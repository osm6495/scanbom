use crate::npm;
use npm::{parse_sbom, Package};
use std::path::PathBuf;
use std::process::Command;
use std::fs::File;
use std::io::BufReader;
use tokio::fs;
use serde_json::Value;
use std::fmt;
use spinners::{Spinner, Spinners};

#[allow(dead_code)]
#[derive(Debug)]
pub enum OutputTypes {
  Json,
  Semgrep,
  Emacs,
  GitlabSast,
  GitlabSecrets,
  JUnitXml,
  Sarif,
  Text,
  Vim,
}

impl fmt::Display for OutputTypes {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let output_str = match self {
          OutputTypes::Json => "--json",
          OutputTypes::Semgrep => "",
          OutputTypes::Emacs => "--emacs",
          OutputTypes::GitlabSast => "--gitlab-sast",
          OutputTypes::GitlabSecrets => "--gitlab-secrets",
          OutputTypes::JUnitXml => "--junit-xml",
          OutputTypes::Sarif => "--sarif",
          OutputTypes::Text => "--text",
          OutputTypes::Vim => "--vim",
      };
      write!(f, "{}", output_str)
  }
}

// Sanitiziation of repo link comes from requirement that the URL comes from npm
async fn clone(repo: &(Package, String)) {
  let name = &repo.0.name;
  let version = &repo.0.version;
  let url = &repo.1;

  Command::new("git")
    .args(["clone", "--branch", version, "--depth", "1", &format!("{}", url), &format!("./packages/{}", name)])
    .output()
    .expect(&format!("Failed to clone npm package with repo: {}", name));
}

async fn clone_bulk(repos: &Vec<(Package, String)>) {
  let mut tasks = Vec::new();

  for repo in repos {
      let package = Package {
        name: repo.0.name.clone(),
        version: repo.0.version.clone(),
      };
      let url= repo.1.clone();
      let task = tokio::spawn(async move {
          clone(&(package, url)).await;
      });
      tasks.push(task);
  }

  for task in tasks {
      task.await.unwrap();
  }
}

async fn clean() -> Result<(), Box<dyn std::error::Error>> {
  fs::remove_dir_all("./packages").await?;
  Ok(())
}


fn scan(output_type: OutputTypes, quiet: bool) -> Result<Value, Box<dyn std::error::Error>> {
  let sp: Option<Spinner> = match quiet { 
    true => None,
    false => Some(Spinner::new(Spinners::BouncingBar, "Scanning with Semgrep".into())),
  };
  let command = match output_type {
    OutputTypes::Semgrep => "semgrep scan --config=auto packages/*".to_string(),
    _ => format!("semgrep scan --config=auto {} -q packages/*", output_type),
  };
  // Need to use sh instead of semgrep directly because if the packages/* arg gets wrapped in quotes the command fails
  let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;

  if let Some(mut spinner) = sp {
    spinner.stop();
  }

  if output.status.success() {
    match output_type {
      OutputTypes::Json => {
        let json_output: Value = serde_json::from_slice(&output.stdout)?;
        Ok(json_output)
    }
    _ => {
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        let json_output = serde_json::Value::String(output_str);
        Ok(json_output)
    }
    }
  } else {
    let error_message = String::from_utf8_lossy(&output.stderr);
    if error_message.len() > 0 {
      Err(format!("Semgrep command failed: {}", error_message).into())
    } else {
      Err(format!("Semgrep command failed: {}", String::from_utf8_lossy(&output.stdout)).into())
    }
  }
}

fn process_scan_output(output: &Value, clean_json: bool) {
  match output {
    Value::String(s) => {
      println!("{}", s);
    }
    _ => {
      if !clean_json {
        println!("{}", output)
      } else {
        if let Some(results) = output["results"].as_array() {
          println!("--- \n");
          for finding in results {
            if let Some(path) = finding["path"].as_str() {
              println!("Finding in package: {}", path);
            }
    
            if let Some(check_id) = finding["check_id"].as_str() {
              println!("Rule: {}", check_id.rsplit('.').next().unwrap());
            }
    
            if let Some(extra) = finding["metadata"].as_object() {
              if let Some(confidence) = extra["confidence"].as_str() {
                  println!("Confidence: {}", confidence);
              }
              if let Some(impact) = extra["impact"].as_str() {
                println!("Impact: {}", impact);
              }
            }
    
            if let Some(extra) = finding["extra"].as_object() {
                if let Some(message) = extra["message"].as_str() {
                    println!("Message: {}", message);
                }
            }
    
            if let (Some(start_line), Some(end_line)) = (
              finding["start"].as_object().and_then(|s| s["line"].as_u64()),
              finding["end"].as_object().and_then(|e| e["line"].as_u64())
            ) {
                println!("Lines: {}-{}", start_line, end_line);
            }
    
            println!("\n--- \n");
          }
        }
      }
    }
  }
}

// Legacy code used when importing JSON instead of an SBOM
fn read_packages_from_file(file_path: &PathBuf) -> Result<Vec<Package>, Box<dyn std::error::Error>> {
  // Open the file
  let file = File::open(file_path)?;
  let reader = BufReader::new(file);
  let json: Value = serde_json::from_reader(reader)?;

  // Deserialize the JSON content into a vector of Package
  let packages: Vec<Package> = json["packages"]
        .as_array()
        .ok_or("Invalid JSON structure")?
        .iter()
        .map(|package| {
            Package {
                name: package["name"].as_str().unwrap_or("").to_string(),
                version: package["version"].as_str().unwrap_or("").to_string(),
            }
        })
        .collect();

  Ok(packages)
}

pub async fn run(clean_json: bool, output_type: OutputTypes, json_input: bool, quiet: bool, input: &PathBuf) {
  //Read from SPDX format SBOM
  let packages = match json_input {
    false => parse_sbom(&input),
    true => read_packages_from_file(&input),
  };
  //let package_names: Vec<String> = packages.clone().into_iter().map(|package| package.name).collect();
  let packages_with_urls = npm::get_package_bulk(&packages.expect("Failed to parse SBOM file"), quiet).await;
  match packages_with_urls {
    Ok(packages_with_urls) => {
      clone_bulk(&packages_with_urls).await;
      match scan(output_type, quiet) {
        Ok(findings) => {
          process_scan_output(&findings, clean_json);
        }
        Err(error) => {
            eprintln!("Error during scanning: {}", error);
        }
      }
      clean().await.unwrap();
    }
    Err(error) => {
      eprintln!("Error: {}", error);
    }
  }
}

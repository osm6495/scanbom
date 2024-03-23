use reqwest::Client;
use serde::{ Deserialize, Serialize };
use anyhow::{Result, Error};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use spinners::{Spinner, Spinners};

#[derive(Debug, Deserialize, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize)]
struct NpmResponse {
    repository: Option<Repository>,
}

#[derive(Deserialize)]
struct Repository {
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SPDX {
    packages: Vec<SPDXPackage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SPDXPackage {
    name: String,
    #[serde(rename = "versionInfo")]
    version: String,
}

pub async fn get_package(package: &Package) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://registry.npmjs.org/{}/{}", package.name, package.version);
    let client = Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let npm_response: NpmResponse = response.json().await?;
        if let Some(repo) = npm_response.repository {
            if let Some(github_url) = clean_github_repo_url(&repo.url){
              Ok(github_url)
            } else {
              Err("GitHub repository couldn't be parsed".into())
            }
        } else {
            Err("GitHub repository not found in npm package metadata".into())
        }
    } else {
        Err(format!("Failed to fetch npm package data: {}", response.status()).into())
    }
}

fn clean_github_repo_url(repo_url: &str) -> Option<String> {
  let mut cleaned_url = repo_url.to_string();

  // Remove the "git+" prefix
  if cleaned_url.starts_with("git+") {
      cleaned_url = cleaned_url[4..].to_string();
  }

  // Remove the "git://" or "git@" prefix
  if cleaned_url.starts_with("git://") {
      cleaned_url = cleaned_url[6..].to_string();
  } else if cleaned_url.starts_with("git@") {
      cleaned_url = cleaned_url[4..].replace(":", "/");
  }

  // Remove the ".git" suffix
  if cleaned_url.ends_with(".git") {
      cleaned_url = cleaned_url[..cleaned_url.len() - 4].to_string();
  }

  // Check if the URL starts with "https://" or "http://"
  if !cleaned_url.starts_with("https://") && !cleaned_url.starts_with("http://") {
      cleaned_url = "https://".to_owned() + &cleaned_url;
  }

  // Check if the URL contains "github.com"
  if cleaned_url.contains("github.com") {
      // Remove the "www." subdomain if present
      cleaned_url = cleaned_url.replace("www.github.com", "github.com");

      Some(cleaned_url)
  } else {
      None
  }
}

pub async fn get_package_bulk(packages: &Vec<Package>, quiet: bool) -> Result<Vec<(Package, String)>, Error> {
  let sp: Option<Spinner> = match quiet { 
    true => None,
    false => Some(Spinner::new(Spinners::BouncingBar, "Cloning Dependencies".into())),
  };
  let mut tasks: Vec<tokio::task::JoinHandle<Result<(String, String, String)>>> = Vec::new();
  let packages_clone = packages.clone();

  for package in packages_clone {
      let package_name = package.name.clone();
      let package_version = package.version.clone();

      let task = tokio::spawn(async move {
          match get_package(&package).await {
              Ok(url) => Ok((package_name, package_version, url)),
              Err(e) => Err(anyhow::anyhow!("Failed to fetch package {}: {}", package_name, e)),
          }
      });

      tasks.push(task);
  }

  let mut results = Vec::new();

  for task in tasks {
      match task.await {
          Ok(result) => match result {
              Ok((package_name, package_version, url)) => {
                  let package = Package {
                      name: package_name,
                      version: package_version,
                  };
                  results.push((package, url));
              }
              Err(e) => {
                  eprintln!("Error: {}", e);
                  return Err(e);
              }
          },
          Err(e) => {
              eprintln!("Task failed: {}", e);
              return Err(anyhow::anyhow!("Task failed: {}", e));
          }
      }
  }

  if let Some(mut spinner) = sp {
    spinner.stop();
  }

  Ok(results)
}

pub fn parse_sbom(file_path: &PathBuf) -> Result<Vec<Package>, Box<dyn std::error::Error>> {
  let mut file = File::open(file_path).expect("Failed to open file");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Failed to read file");

  let spdx: SPDX = serde_json::from_str(&contents).expect("Failed to parse SPDX SBOM");

  let packages = spdx.packages
      .into_iter()
      .filter_map(|pkg| {
          if pkg.name.starts_with("npm:") {
              let name = pkg.name.trim_start_matches("npm:").to_string();
              Some(Package {
                  name,
                  version: pkg.version,
              })
          } else {
              None
          }
      })
      .collect();
    
  Ok(packages)
}
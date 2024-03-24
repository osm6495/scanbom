use clap::{Command, Arg};
use scan::OutputTypes;
use std::time::Instant;

mod scan;
mod npm;

#[tokio::main]
async fn main() {
  let matches = Command::new("SBOM vulnerability scanner")
    .version("0.1")
    .author("Owen McCarthy")
    .about("Parse SBOM files and scan all dependencies with Semgrep")
    .arg(
      Arg::new("clean_json")
        .short('c')
        .long("clean-json")
        .help("Output simplified JSON format")
        .action(clap::ArgAction::SetTrue),
    )
    .arg(
      Arg::new("json_input")
        .short('j')
        .long("json-input")
        .help("Use JSON input format")
        .action(clap::ArgAction::SetTrue),
    )
    .arg(
      Arg::new("timer")
        .short('t')
        .long("timer")
        .help("Measure elapsed time")
        .action(clap::ArgAction::SetTrue),
    )
    .arg(
      Arg::new("quiet")
        .short('q')
        .long("quiet")
        .help("Only print output, with no loading bar")
        .action(clap::ArgAction::SetTrue),
    )
    .arg(
      Arg::new("output_type")
        .short('o')
        .long("output-type")
        .help("Specify the output format")
        .value_parser(["json", "semgrep", "emacs", "gitlab-sast", "gitlab-secrets", "junit-xml", "sarif", "text", "vim"])
        .default_value("semgrep"),
    )
    .arg(
      Arg::new("file_path")
        .help("Path to the SBOM file")
        .required(true)
        .value_parser(clap::value_parser!(std::path::PathBuf)),
    )
    .get_matches();

  let clean_json = *matches.get_one::<bool>("clean_json").unwrap_or(&false);
  let json_input = *matches.get_one::<bool>("json_input").unwrap_or(&false);
  let file_path = matches.get_one::<std::path::PathBuf>("file_path").unwrap();
  let timer = *matches.get_one::<bool>("timer").unwrap_or(&false);
  let quiet = *matches.get_one::<bool>("quiet").unwrap_or(&false);

  let output_type = match matches.get_one::<String>("output_type").map(|s| s.as_str()) {
    Some("json") => OutputTypes::Json,
    Some("semgrep") => OutputTypes::Semgrep,
    Some("emacs") => OutputTypes::Emacs,
    Some("gitlab-sast") => OutputTypes::GitlabSast,
    Some("gitlab-secrets") => OutputTypes::GitlabSecrets,
    Some("junit-xml") => OutputTypes::JUnitXml,
    Some("sarif") => OutputTypes::Sarif,
    Some("text") => OutputTypes::Text,
    Some("vim") => OutputTypes::Vim,
    _ => unreachable!(),
  };

  let start_time = Instant::now();
  scan::run(clean_json, output_type, json_input, quiet, file_path).await;
  let elapsed_time = start_time.elapsed();
  if timer {
    println!("Elapsed time: {:?}", elapsed_time);
  }
}
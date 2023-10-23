use std::{fs, path::PathBuf, process::exit};

use clap::Parser;
use glob::glob;

use zh_formatter::{config::Config, run_markdown};

/// A formatter tool for Chinese text content.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File pattern
    path: Option<String>,

    /// Config file path, .zh-formatter-config.toml by default
    #[arg(short, long)]
    config: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let config_file = args
        .config
        .unwrap_or(PathBuf::from(".zh-formatter-config.toml"));
    let config: Config = match fs::read_to_string(config_file) {
        Ok(config_file) => {
            toml::from_str(&config_file).expect("Config file is not a legal TOML file.")
        }
        Err(e) => {
            println!(
                "Unable to read config file, using the default config: {}",
                e
            );
            toml::from_str("").unwrap()
        }
    };

    let path_pattern = args.path.unwrap_or("./**/*.md".to_string());
    for file in glob(&path_pattern).expect("Path pattern error.") {
        match file {
            Ok(file) => {
                println!("Formatting: {}", file.to_str().unwrap());

                let file_content = match fs::read_to_string(&file) {
                    Ok(s) => s,
                    Err(e) => {
                        println!("Unable to read file: {}", e);
                        exit(1);
                    }
                };

                if let Err(e) = fs::write(&file, run_markdown(&file_content, &config)) {
                    println!("Unable to write file: {}", e);
                    exit(1);
                }
            }
            Err(e) => {
                println!("Can not read file: {}", e);
                exit(1);
            }
        }
    }
}

use std::{fs, path::PathBuf, process::exit};

use clap::Parser;
use glob::glob;

use zhlint::{config::Config, run};

/// A formatter tool for Chinese text content.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File pattern
    #[arg(default_value_t = String::from("./**/*.md"))]
    path: String,

    /// Config file path, .zhlintrc.toml by default
    #[arg(short, long, default_value_os_t = PathBuf::from(".zhlintrc.toml"))]
    config: PathBuf,
}

fn main() {
    let args = Args::parse();

    let config: Config = match fs::read_to_string(args.config) {
        Ok(config_file) => {
            toml::from_str(&config_file).expect("Config file is not a legal TOML file.")
        }
        Err(e) => {
            println!("Unable to read config file, using the default config: {e}");
            Config::default()
        }
    };

    for path in glob(&args.path).expect("Path pattern error.") {
        match path {
            Ok(path) => {
                if !path.is_file() {
                    continue;
                }

                println!("Formatting: {}", path.to_str().unwrap());

                let file_content = match fs::read_to_string(&path) {
                    Ok(s) => s,
                    Err(e) => {
                        println!("Unable to read file: {e}");
                        exit(1);
                    }
                };

                let report = run(&file_content, &config);

                for error in report.errors.clone().into_iter().rev() {
                    let error = miette::Error::new(error).with_source_code(file_content);
                    println!("{:?}", error);
                }

                if let Err(e) = fs::write(&path, report.text) {
                    println!("Unable to write file: {}", e);
                    exit(1);
                }
            }
            Err(e) => {
                println!("Can not read file: {e}");
                exit(1);
            }
        }
    }
}

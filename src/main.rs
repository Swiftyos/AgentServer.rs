use clap::{Parser, Subcommand};
use serde::Deserialize;
use serde_with_expand_env::with_expand_envs;
use std::fs;
use tracing::{Level, info, error, warn, debug, trace};
use tracing_subscriber::FmtSubscriber;

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(deserialize_with = "with_expand_envs")]
    log_level: String,
    #[serde(deserialize_with = "with_expand_envs")]
    modules_directory: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

#[derive(Subcommand)]
enum Commands {
    Run,
    Check,
    // Add other subcommands as needed
}

fn load_config(file_path: &str) -> Result<Config, serde_yaml::Error> {
    let file_contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Failed to read file contents");
            std::process::exit(1);
        }
    };

    let config: Config = serde_yaml::from_str(&file_contents)?;
    Ok(config)
}

fn setup_logging(log_level: &str) {
    let level = match log_level {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");
}

fn main() {
    let cli = Cli::parse();

    // Load the configuration file
    let config = match load_config(&cli.config) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load configuration file: {}", e);
            std::process::exit(1);
        }
    };

    // Set up logging
    setup_logging(&config.log_level);

    // Access the modules directory from the configuration
    let modules_directory = &config.modules_directory;

    info!("Modules directory: {}", modules_directory);
    info!("Log level: {}", config.log_level);

    // Handle subcommands
    match cli.command {
        Some(Commands::Run) => {
            // Run your application logic here
            // ...
            info!("Running application logic");
        }
        Some(Commands::Check) => {
            // Check your application logic here
            // ...
            info!("Checking configuration");
            trace!("Trace message");
            debug!("Debug message");
            info!("Info message");
            warn!("Warn message");
            error!("Error message");

        }
        None => {
            // No subcommand provided
            // ...
            warn!("No subcommand provided")
        }
    }
}
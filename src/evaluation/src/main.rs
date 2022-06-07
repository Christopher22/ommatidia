use std::io::Read;

use ommatidia::{ErrorHandler, Estimates};

mod config;
mod error_printer;

fn load_config() -> Result<config::Config, String> {
    let mut input = std::io::stdin();
    let mut toml_content = String::default();
    input
        .read_to_string(&mut toml_content)
        .map_err(|error| format!("Unable to read TOML input: {}", error))?;
    toml::from_str(toml_content.as_str())
        .map_err(|error| format!("Unable to parse configuration: {}", error))
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let error_handler = error_printer::ErrorPrinter::default();
    let config = match load_config() {
        Ok(config) => config,
        Err(error) => {
            error_handler.handle_raw(format!("unable to load config from TOML: {}", error));
            return;
        }
    };

    let mut config = match config.try_spawn().await {
        Ok(config) => config,
        Err(error) => {
            error_handler.handle_raw(format!("unable to create detectors: {}", error));
            return;
        }
    };

    // Print out all the estimates
    let mut cache = Vec::with_capacity(32);
    for dataset in config.datasets {
        for estimate in Estimates::load(&mut config.detectors, dataset, &error_handler).await {
            serde_json::to_writer(&mut cache, &estimate).expect("valid object");
            println!("{}", std::str::from_utf8(&cache).expect("valid utf-8"));
            cache.clear();
        }
    }

    // Print all error occuring during shutdown of detectors.
    for shutdown_error in config.detectors.shutdown().await {
        error_handler.handle(shutdown_error);
    }
}

use std::io::Read;

mod config;

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
    let config = match load_config() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };

    let config = match config.try_spawn().await {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Unable to create detectors: {}", error);
            return;
        }
    };

    /*for source in config.files {
        config.detectors.
        source.
    }*/
}

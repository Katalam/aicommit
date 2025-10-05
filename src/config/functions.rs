use config::Config;
use std::fs;
use crate::config::structs::Provider;

pub fn load_config() -> Config {
    let config_path = get_config_path();

    Config::builder()
        .add_source(config::File::from(config_path).required(true))
        .add_source(config::Environment::with_prefix("AICOMMIT"))
        .build().unwrap()
}

pub fn load_serialized_config() -> crate::config::structs::Config {
    load_config()
        .try_deserialize::<crate::config::structs::Config>()
        .expect("Could not parse config file. Check the file format.")
}

fn get_config_path() -> std::path::PathBuf {
    let home_dir = dirs::home_dir()
        .expect("Could not find home directory");

    home_dir.join(".aicommit/config.json")
}

pub fn copy_default_config() -> bool {
    ensure_directory_exists(get_config_path().as_path());

    let default_config = fs::read_to_string("./src/stubs/config.json")
        .expect("Could not read config.json");

    fs::write(get_config_path(), default_config).expect("Could not write default config.json");

    true
}

fn ensure_directory_exists(path: &std::path::Path) {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .expect("Failed to create configuration directory");
        }
    }
}

pub fn api_key_exist() -> bool {
    !get_provider().api_key.is_empty()
}

pub fn get_provider() -> Provider {
    let config = load_serialized_config();
    let default_provider = &config.default_provider;

    config.providers.into_iter()
        .find(|p| p.name == default_provider.as_str())
        .unwrap()
}

pub fn check_config_exists() -> bool {
    get_config_path().exists()
}

pub fn validate_config() -> bool {
    load_config()
        .try_deserialize::<crate::config::structs::Config>()
        .expect("Could not parse config file. Check the file format.");

    true
}
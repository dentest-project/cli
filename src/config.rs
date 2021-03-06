use std::fs;
use yaml_rust::{Yaml, YamlLoader};

pub fn extract_config_value(config_file: &String, property: &str) -> Yaml {
    let config = &YamlLoader::load_from_str(
        &fs::read_to_string(config_file).expect("Could not read configuration file"),
    )
    .expect("Could not parse configuration file")[0];

    return config[property].clone();
}

#[cfg(debug_assertions)]
pub fn get_api_url() -> String {
    String::from("http://api.entest.local")
}

#[cfg(not(debug_assertions))]
pub fn get_api_url() -> String {
    String::from("https://api.ente.st")
}

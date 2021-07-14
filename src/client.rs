use crate::config;
use crate::entities::feature_list_item::FeatureListItem;
use crate::error::Error;
use crate::utils::json::{convert_value_vec_to_string_vec, deserialize_vec};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::env;
use termion::{color, style};

fn get<T: DeserializeOwned>(url: &str, fetch_msg: &str, decode_msg: &str) -> Result<T, Error> {
    let token = match retrieve_token() {
        Ok(t) => t,
        Err(e) => {
            return Err(e);
        }
    };
    let client = reqwest::blocking::Client::new();

    let response = match client
        .get(format!("{}/{}", config::get_api_url(), url).as_str())
        .header("Authorization", format!("Pull {}", token).as_str())
        .send()
    {
        Ok(r) => r,
        Err(_) => {
            return Err(Error::new(String::from(fetch_msg)));
        }
    };

    match response.json() {
        Ok(j) => Ok(j),
        Err(_) => Err(Error::new(String::from(decode_msg))),
    }
}

fn retrieve_token() -> Result<String, Error> {
    match env::var("DENTEST_TOKEN") {
        Ok(t) => Ok(t),
        Err(_) => Err(Error::new(String::from(
            "Could not retrieve the DENTEST_TOKEN env var",
        ))),
    }
}

pub fn retrieve_paths() -> Result<Vec<String>, Error> {
    println!(
        "{}{}Fetching{} paths",
        style::Bold,
        color::Fg(color::Cyan),
        style::Reset
    );
    let response: Value = match get(
        "pull/paths",
        "Could not retrieve paths",
        "Could not decode the response from server while retrieving paths",
    ) {
        Ok(r) => r,
        Err(e) => {
            return Err(e);
        }
    };

    match response {
        Value::Array(v) => Ok(convert_value_vec_to_string_vec(v)),
        _ => Err(Error::new(String::from("Could not retrieve paths"))),
    }
}

pub fn retrieve_features() -> Result<Vec<FeatureListItem>, Error> {
    println!(
        "{}{}Fetching{} features",
        style::Bold,
        color::Fg(color::Cyan),
        style::Reset
    );
    let response: Value = match get(
        "pull/features",
        "Could not retrieve features",
        "Could not decode the response from server while retrieving features",
    ) {
        Ok(r) => r,
        Err(e) => {
            return Err(e);
        }
    };

    match response {
        Value::Array(v) => Ok(deserialize_vec(v)),
        _ => Err(Error::new(String::from("Could not retrieve features"))),
    }
}

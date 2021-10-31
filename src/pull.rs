use crate::client;
use crate::config::extract_config_value;
use crate::entities::feature_list_item::FeatureListItem;
use crate::error::Error;
use std::fs;
use termion::{color, style};
use yaml_rust::Yaml;

pub fn pull(config_file: String) -> Result<(), Error> {
    let dir = match get_dir(&config_file) {
        Ok(s) => s,
        Err(e) => {
            return Err(e);
        }
    };
    let inline_parameter_wrapper = get_inline_parameter_wrapper(&config_file);
    match clear_dir(&dir) {
        Ok(_) => (),
        Err(e) => {
            return Err(e);
        }
    };
    match retrieve_paths(&dir) {
        Ok(_) => (),
        Err(e) => {
            return Err(e);
        }
    };
    match retrieve_features(&dir, inline_parameter_wrapper) {
        Ok(_) => (),
        Err(e) => {
            return Err(e);
        }
    };

    return Ok(());
}

fn get_dir(config_file: &String) -> Result<String, Error> {
    match extract_config_value(&config_file, "dir") {
        Yaml::String(s) => Ok(s),
        _ => Err(Error::new(format!(
            "Please add a \"dir\" property to {}",
            config_file
        ))),
    }
}

fn get_inline_parameter_wrapper(config_file: &String) -> String {
    match extract_config_value(&config_file, "inline_parameter_wrapper") {
        Yaml::String(s) => s,
        _ => String::from(""),
    }
}

fn clear_dir(dir: &String) -> Result<(), Error> {
    match fs::metadata(dir) {
        Ok(m) => match m.is_dir() {
            true => (),
            false => return Err(Error::new(format!("\"{}\" is not a directory", dir))),
        },
        Err(_) => {
            return Ok(());
        }
    }

    println!(
        "{}{}Clearing{} \"{}\" directory",
        style::Bold,
        color::Fg(color::Yellow),
        style::Reset,
        dir
    );
    match fs::remove_dir_all(dir) {
        Ok(_) => Ok(()),
        Err(_) => {
            return Err(Error::new(format!(
                "Could not remove the \"{}\" directory",
                dir
            )));
        }
    }
}

fn retrieve_paths(dir: &String) -> Result<(), Error> {
    let result = match client::retrieve_paths() {
        Ok(v) => v,
        Err(e) => {
            return Err(e);
        }
    };

    match write_paths(dir, result) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn write_paths(dir: &String, paths: Vec<String>) -> Result<(), Error> {
    for i in paths.iter() {
        println!(
            "{}{}Creating{} path {}{}{}",
            style::Bold,
            color::Fg(color::Green),
            style::Reset,
            style::Italic,
            i,
            style::Reset
        );
        match fs::create_dir_all(format!("{}/{}", dir, i)) {
            Ok(_) => (),
            Err(_) => {
                return Err(Error::new(format!(
                    "Could not write to \"{}\" directory",
                    dir
                )));
            }
        };
    }

    Ok(())
}

fn retrieve_features(dir: &String, inline_parameter_wrapper: String) -> Result<(), Error> {
    let result = match client::retrieve_features(inline_parameter_wrapper) {
        Ok(v) => v,
        Err(e) => {
            return Err(e);
        }
    };

    match write_features(dir, result) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn write_features(dir: &String, features: Vec<FeatureListItem>) -> Result<(), Error> {
    for i in features.iter() {
        println!(
            "{}{}Creating{} feature {}{}{}",
            style::Bold,
            color::Fg(color::Green),
            style::Reset,
            style::Italic,
            i.path,
            style::Reset
        );
        match fs::write(format!("{}/{}", dir, i.path), &i.feature) {
            Ok(_) => (),
            Err(_) => {
                return Err(Error::new(format!(
                    "Could not write to \"{}\" directory",
                    dir
                )));
            }
        };
    }

    Ok(())
}

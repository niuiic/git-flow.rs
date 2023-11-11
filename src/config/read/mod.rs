use std::{fs::File, io::Read};

use anyhow::{bail, Result};

use super::{definition, path};

#[cfg(test)]
mod test;

pub fn read_config() -> Result<Vec<definition::Config>> {
    let config_path_list = path::get_config_path_list()?;

    let mut config_file = None;
    for config_path in config_path_list {
        if let Ok(config_file_v) = File::open(config_path) {
            config_file = Some(config_file_v);
            break;
        }
    }
    if config_file.is_none() {
        bail!("Config not found");
    }

    let mut text = String::new();
    config_file.unwrap().read_to_string(&mut text)?;

    let config_list: Vec<definition::Config> = serde_json::from_str(&text)?;

    Ok(config_list)
}

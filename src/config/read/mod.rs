use std::{fs::File, io::Read, path::PathBuf};

use anyhow::{bail, Context, Result};

use super::{definition, path, validate::validate_config};

#[cfg(test)]
mod test;

pub fn read_config(config_path: Option<PathBuf>) -> Result<definition::Config> {
    // -- get path --
    let config_path_list = match config_path {
        Some(config_path_v) => vec![config_path_v],
        None => path::get_config_path_list().context("unable to get default config path")?,
    };

    // -- read file --
    let mut config_file = None;
    for config_path in config_path_list {
        if let Ok(config_file_v) = File::open(config_path) {
            config_file = Some(config_file_v);
            break;
        }
    }
    if config_file.is_none() {
        bail!("config file is not found");
    }
    let mut text = String::new();
    config_file.unwrap().read_to_string(&mut text)?;

    // -- parse config --
    let config = toml::from_str::<definition::Config>(&text).context("unable to parse config")?;

    // -- validate --
    match validate_config(&config) {
        Ok(_) => Ok(config),
        Err(err) => Err(err.context("config is invalid")),
    }
}

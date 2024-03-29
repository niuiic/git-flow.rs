use std::path::PathBuf;

use crate::{config::read::read_config, echo::Echo};

pub fn check_config(config_path: PathBuf) {
    match read_config(Some(config_path)) {
        Ok(_) => Echo::success("config is valid"),
        Err(err) => {
            Echo::error("config is invalid");
            eprintln!("\n{:?}", err);
        }
    }
}

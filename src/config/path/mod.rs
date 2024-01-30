use std::{env, path::PathBuf};

use anyhow::{anyhow, bail, Result};

#[cfg(test)]
mod test;

fn get_global_config_path() -> Result<String> {
    match env::consts::OS {
        "windows" => {
            let data_dir =
                env::var_os("APPDATA").ok_or(anyhow!("system env arg 'APPDATA' is missing"))?;
            Ok(PathBuf::from(data_dir)
                .join("git-flow/config.toml")
                .to_str()
                .unwrap()
                .to_string())
        }
        _ => {
            let home_dir =
                env::var_os("HOME").ok_or(anyhow!("system env arg 'HOME' is missing"))?;
            Ok(PathBuf::from(home_dir)
                .join(".config/git-flow/config.toml")
                .to_str()
                .unwrap()
                .to_string())
        }
    }
}

fn get_local_config_path() -> Result<String> {
    let mut cur_dir = env::current_dir()?;

    while !cur_dir.parent().is_none() {
        let git_dir = cur_dir.join(".git");
        if git_dir.exists() && git_dir.is_dir() {
            return Ok(cur_dir.join(".git-flow.toml").to_str().unwrap().to_string());
        }
        cur_dir.pop();
    }

    bail!("git root is not found")
}

pub fn get_config_path_list() -> Result<Vec<String>> {
    Ok(vec![get_local_config_path()?, get_global_config_path()?])
}

use std::collections::HashSet;

use anyhow::{anyhow, Ok, Result};

use super::definition::Config;

#[cfg(test)]
mod test;

pub fn validate_config(config_list: &Vec<Config>) -> Result<()> {
    let mut duplicate_config_types = HashSet::<String>::new();
    let mut duplicate_config_names = HashSet::<String>::new();

    // check duplicate config
    for i in 0..config_list.len() {
        let config = &config_list[i];

        for j in 0..config_list.len() {
            let config_2 = &config_list[j];

            if i == j {
                continue;
            }
            if config.branch_type == config_2.branch_type {
                duplicate_config_types.insert(config.branch_type.clone());
            }
            if config.branch_name == config_2.branch_name {
                duplicate_config_names.insert(config.branch_name.clone());
            }
        }
    }

    if duplicate_config_types.len() + duplicate_config_names.len() == 0 {
        return Ok(());
    }

    if duplicate_config_types.len() > 0 && duplicate_config_names.len() > 0 {
        Err(anyhow!(
            "Invalid config: duplicate type {} and duplicate name {}",
            duplicate_config_types
                .into_iter()
                .collect::<Vec<String>>()
                .join(","),
            duplicate_config_names
                .into_iter()
                .collect::<Vec<String>>()
                .join(","),
        ))
    } else if duplicate_config_names.len() == 0 {
        Err(anyhow!(
            "Invalid config: duplicate type {}",
            duplicate_config_types
                .into_iter()
                .collect::<Vec<String>>()
                .join(","),
        ))
    } else {
        Err(anyhow!(
            "Invalid config: duplicate name {}",
            duplicate_config_names
                .into_iter()
                .collect::<Vec<String>>()
                .join(","),
        ))
    }
}

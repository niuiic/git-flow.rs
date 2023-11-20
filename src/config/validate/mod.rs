use anyhow::{bail, Ok, Result};
use regex::Regex;

use super::definition::Config;

#[cfg(test)]
mod test;

pub fn validate_config(config_list: &Vec<Config>) -> Result<()> {
    has_duplicate_config(&config_list)?;
    has_invalid_branch_name(&config_list)?;
    target_is_valid_regex(&config_list)?;
    Ok(())
}

fn has_duplicate_config(config_list: &Vec<Config>) -> Result<()> {
    for i in 0..config_list.len() {
        let config = &config_list[i];

        for j in 0..config_list.len() {
            let config_2 = &config_list[j];

            if i == j {
                continue;
            }
            if config.branch_type == config_2.branch_type {
                bail!(format!(
                    "Invalid config: duplicate type {}",
                    &config.branch_type
                ));
            }
            if config.branch_name == config_2.branch_name {
                bail!(format!(
                    "Invalid config: duplicate name {}",
                    &config.branch_type
                ));
            }
        }
    }

    Ok(())
}

fn has_invalid_branch_name(config_list: &Vec<Config>) -> Result<()> {
    let regex = Regex::new(r"\{new_branch\}").unwrap();

    for i in 0..config_list.len() {
        let config = &config_list[i];

        let matches: Vec<_> = regex.find_iter(&config.branch_name).collect();
        if matches.len() == 0 {
            bail!(format!(
                "Invalid config: {} does not contain a {{new_branch}}",
                &config.branch_name
            ));
        }
        if matches.len() > 1 {
            bail!(format!(
                "Invalid config: {} contains more than one {{new_branch}}",
                &config.branch_name
            ));
        }
    }

    Ok(())
}

fn target_is_valid_regex(config_list: &Vec<Config>) -> Result<()> {
    for i in 0..config_list.len() {
        let config = &config_list[i];

        for j in 0..config.target_branches.len() {
            let target = &config.target_branches[j];

            if Regex::new(&target.name).is_err() {
                bail!(format!(
                    "Invalid config: target branch {} is not a valid regex",
                    &target.name
                ))
            }
        }
    }

    Ok(())
}

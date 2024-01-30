use anyhow::{bail, Ok, Result};
use regex::Regex;

use super::definition::Config;

#[cfg(test)]
mod test;

pub fn validate_config(config: &Config) -> Result<()> {
    no_duplicate_config(&config)?;
    target_is_valid_regex(&config)?;
    Ok(())
}

fn no_duplicate_config(config: &Config) -> Result<()> {
    let branch_types = &config.branch_types;

    for i in 0..branch_types.len() {
        let branch_type_i = &branch_types[i];

        for j in 0..i {
            let branch_type_j = &branch_types[j];

            if branch_type_i.name == branch_type_j.name {
                bail!(format!(
                    "invalid config: duplicate branch type name {}",
                    &branch_type_i.name
                ));
            }

            if branch_type_i.create == branch_type_j.create {
                bail!(format!(
                    "invalid config: duplicate branch type create {}",
                    &branch_type_i.create
                ));
            }
        }
    }

    Ok(())
}

fn target_is_valid_regex(config: &Config) -> Result<()> {
    for i in 0..config.branch_types.len() {
        let branch_type = &config.branch_types[i];

        for j in 0..branch_type.to.len() {
            let target = &branch_type.to[j];

            if Regex::new(&target.name).is_err() {
                bail!(format!(
                    "invalid config: target branch {} is not a valid regex",
                    &target.name
                ))
            }
        }
    }

    Ok(())
}

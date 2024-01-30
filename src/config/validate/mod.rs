use anyhow::{bail, Result};
use regex::Regex;

use super::definition::Config;

#[cfg(test)]
mod test;

pub fn validate_config(config: &Config) -> Result<()> {
    no_duplicate_branch_type(&config)?;
    target_is_valid_regex(&config)?;
    create_is_valid(&config)?;
    Ok(())
}

fn no_duplicate_branch_type(config: &Config) -> Result<()> {
    let branch_types = &config.branch_types;

    for i in 0..branch_types.len() {
        let branch_type_i = &branch_types[i];

        for j in 0..i {
            let branch_type_j = &branch_types[j];

            if branch_type_i.name == branch_type_j.name {
                bail!(
                    "invalid config: duplicate branch type name {}",
                    &branch_type_i.name
                );
            }

            if branch_type_i.create == branch_type_j.create {
                bail!(
                    "invalid config: duplicate branch type create {}",
                    &branch_type_i.create
                );
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

fn create_is_valid(config: &Config) -> Result<()> {
    let invalid_creates = config
        .branch_types
        .iter()
        .filter(|x| {
            x.create
                .match_indices("{new_branch}")
                .map(|x| x.1.to_string())
                .collect::<Vec<String>>()
                .len()
                != 1
        })
        .map(|x| format!("{}: {}", x.name, x.create))
        .collect::<Vec<String>>();

    if invalid_creates.len() > 0 {
        bail!("These branch_types have invalid 'create' which should include only one {{new_branch}}:\n{}", invalid_creates.join("\n"))
    }

    Ok(())
}

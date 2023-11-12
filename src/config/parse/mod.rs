use anyhow::{bail, Result};
use regex::Regex;

use super::definition::Config;

#[cfg(test)]
mod test;

/// get branch type and branch name from arguments
/// return (branch_type, branch_name)
pub fn get_branch_type_name(
    config_list: &Vec<Config>,
    args: &Vec<String>,
) -> Result<(String, String)> {
    if args.len() == 0 {
        bail!("No enough arguments");
    } else if args.len() == 1 {
        let config = config_list.iter().find(|config| {
            let branch_name = config.branch_name.replace("{new_branch}", ".*");
            let regex = Regex::new(&format!("^{}$", branch_name)).unwrap();
            regex.is_match(&args[0])
        });

        if let Some(config) = config {
            let branch_name = &args[0].replace(&config.branch_name.replace("{new_branch}", ""), "");
            Ok((config.branch_type.to_string(), branch_name.to_string()))
        } else {
            bail!("No matched branch type found");
        }
    } else {
        let config = config_list
            .iter()
            .find(|x| x.branch_type.as_str() == args[0]);
        if config.is_none() {
            bail!("No matched branch type found");
        }

        Ok((args[0].to_string(), args[1].to_string()))
    }
}

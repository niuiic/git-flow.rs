use std::{path::PathBuf, process};

use anyhow::{bail, Result};
use regex::Regex;

use crate::{
    config::{
        definition::{BranchType, Command, BRANCH_NAME_PLACEHOLDER},
        read::read_config,
    },
    echo::Echo,
    git::Git,
};

pub fn env_valid() -> bool {
    if !Git::git_installed() {
        Echo::error("git command is not found");
        return false;
    }

    if !Git::in_git_project() {
        Echo::error("not in a git project");
        return false;
    }

    true
}

pub fn get_branch_type_name(
    branch_name: String,
    branch_type: Option<String>,
    config_path: Option<PathBuf>,
) -> Result<(
    /* branch_name */ String,
    /* branch_type */ BranchType,
)> {
    let config = match read_config(config_path) {
        Err(err) => return Err(err),
        Ok(config_v) => config_v,
    };

    if let Some(branch_type_v) = branch_type {
        let target_branch_type = config.branch_types.iter().find(|x| x.name == branch_type_v);
        match target_branch_type {
            None => bail!("no matched branch type"),
            Some(target_branch_type_v) => {
                return Ok((
                    target_branch_type_v
                        .create
                        .replace(BRANCH_NAME_PLACEHOLDER, &branch_name),
                    target_branch_type_v.clone(),
                ))
            }
        }
    }

    let target_branch_type = config.branch_types.iter().find(|x| {
        let regex = Regex::new(&format!(
            "^{}$",
            x.create.replace(BRANCH_NAME_PLACEHOLDER, ".*")
        ))
        .unwrap();
        regex.is_match(&branch_name)
    });
    match target_branch_type {
        None => bail!("no matched branch type"),
        Some(target_branch_type_v) => Ok((branch_name, target_branch_type_v.clone())),
    }
}

pub fn run_hook(command: Option<Command>) -> Result<()> {
    let command = match command {
        Some(command_v) => command_v,
        None => return Ok(()),
    };

    // -- print start --
    let msg = format!("Run hook: {} {}", command.command, command.args.join(" "));
    let finish = Echo::progress(&msg);

    // -- run --
    let result = process::Command::new(command.command)
        .args(command.args)
        .output();

    // -- print result --
    let output = match result {
        Err(err) => {
            finish(false, &err.to_string());
            bail!("");
        }
        Ok(output_v) => output_v,
    };
    match output.status.success() {
        false => {
            finish(false, &String::from_utf8(output.stderr).unwrap());
            bail!("");
        }
        true => {
            finish(true, &msg);
            Ok(())
        }
    }
}
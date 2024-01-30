use std::io;

use anyhow::{bail, Result};

use super::Cli;
use crate::{echo::Echo, git::Git};

pub enum SyncTarget {
    Remote,
    Local,
}

pub enum SyncStrategy {
    Override,
    Increment,
}

impl Cli {
    pub fn sync(target: SyncTarget, strategy: SyncStrategy) {
        // %% fetch remote branches %%
        let stop = Echo::progress("Fetch remote branches");
        let result = Git::fetch_remote_branches();
        stop();
        if let Err(err) = result {
            println!();
            Echo::error(&err.to_string());
            return;
        }
        print!("\r");
        Echo::success("Fetch remote branches");

        // %% select remote repo %%
        let repo = match select_repo() {
            Err(err) => {
                Echo::error(&err.to_string());
                return;
            }
            Ok(value) => value,
        };

        // %% get branches %%
        let local_branches = match Git::get_local_branches() {
            Err(err) => {
                Echo::error(&err.to_string());
                return;
            }
            Ok(value) => value,
        };
        let remote_branches = match Git::get_remote_branches(&repo) {
            Err(err) => {
                Echo::error(&err.to_string());
                return;
            }
            Ok(value) => value,
        };

        // %% sync branches %%
        sync_branches(&repo, &target, &strategy, &local_branches, &remote_branches);
    }
}

fn select_repo() -> Result<String> {
    let repos = match Git::get_remote_repos() {
        Err(err) => {
            return Err(err);
        }
        Ok(value) => value,
    };

    if repos.len() == 0 {
        bail!("No remote repo specified");
    }

    if repos.len() == 1 {
        return Ok(repos[0].clone());
    }

    println!("Select which repo to sync branches:");
    repos
        .iter()
        .enumerate()
        .for_each(|(i, x)| println!("[{}] {}", i + 1, x));

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let index = match input.trim().parse::<usize>() {
        Err(_) => bail!("Invalid input"),
        Ok(value) => value,
    };
    if index > repos.len() {
        bail!("Invalid input");
    }
    Ok(repos[index].clone())
}

fn sync_branches(
    repo: &str,
    target: &SyncTarget,
    strategy: &SyncStrategy,
    local_branches: &Vec<String>,
    remote_branches: &Vec<String>,
) {
    let source_branches = match target {
        SyncTarget::Local => remote_branches,
        SyncTarget::Remote => local_branches,
    };

    let target_branches = match target {
        SyncTarget::Local => local_branches,
        SyncTarget::Remote => remote_branches,
    };

    // %% check diff %%
    if source_branches.len() == target_branches.len()
        && source_branches
            .iter()
            .all(|x| target_branches.iter().any(|y| x.as_str() == y))
    {
        Echo::info("No branch to sync");
        return;
    }

    if let SyncStrategy::Override = strategy {
        let redundant_branches = target_branches
            .iter()
            .filter(|x| !source_branches.iter().any(|y| x.as_str() == y))
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        if redundant_branches.len() == 0 {
            Echo::info("No redundant branches");
        } else {
            let stop = Echo::progress("Delete redundant branches");
            for branch in &redundant_branches {
                if let Err(err) = del_branch(target, repo, branch) {
                    stop();
                    println!();
                    Echo::error(&err.to_string());
                    return;
                };
            }
            stop();
            print!("\r");
            Echo::success(&format!(
                "Delete redundant branches: {}",
                &redundant_branches.join(",")
            ));
        }
    }

    let missing_branches = source_branches
        .iter()
        .filter(|x| !target_branches.iter().any(|y| x.as_str() == y))
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    if missing_branches.len() == 0 {
        Echo::info("No missing branches");
        return;
    }

    let stop = Echo::progress("Create missing branches");
    for branch in &missing_branches {
        if let Err(err) = create_branch(target, repo, branch) {
            stop();
            println!();
            Echo::error(&err.to_string());
            return;
        };
    }
    stop();
    print!("\r");
    Echo::success(&format!(
        "Create missing branches: {}",
        &missing_branches.join(",")
    ));
}

fn del_branch(target: &SyncTarget, repo: &str, branch: &str) -> Result<()> {
    match target {
        SyncTarget::Remote => Git::del_remote_branch(repo, branch),
        SyncTarget::Local => Git::del_local_branch(branch),
    }
}

fn create_branch(target: &SyncTarget, repo: &str, branch: &str) -> Result<()> {
    match target {
        SyncTarget::Remote => Git::create_remote_branch(repo, branch, branch),
        SyncTarget::Local => Git::create_local_branch(&format!("{}/{}", repo, branch), branch),
    }
}

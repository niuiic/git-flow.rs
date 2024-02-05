use std::io;

use anyhow::{bail, Context, Result};

use crate::{
    cli::{SyncStrategy, SyncTarget},
    echo::Echo,
    git::Git,
};

pub fn sync_repo_branches(target: SyncTarget, strategy: SyncStrategy) {
    // -- fetch remote data --
    let finish = Echo::progress("fetch remote data");
    let result = Git::fetch_remote_data();
    finish();
    match result {
        Err(err) => {
            println!();
            Echo::error(err.to_string());
            return;
        }
        Ok(_) => {
            Echo::success("\rfetch remote data");
        }
    }

    // -- select remote repo --
    let repo = match select_repo() {
        Err(err) => {
            Echo::error(err.to_string());
            return;
        }
        Ok(value) => value,
    };

    // -- get branches --
    let local_branches = match Git::get_local_branches() {
        Err(err) => {
            Echo::error(err.to_string());
            return;
        }
        Ok(value) => value,
    };
    let remote_branches = match Git::get_remote_branches(&repo) {
        Err(err) => {
            Echo::error(err.to_string());
            return;
        }
        Ok(value) => value,
    };

    // -- sync branches --
    sync_branches(&repo, &target, &strategy, &local_branches, &remote_branches);
}

fn select_repo() -> Result<String> {
    // -- fetch repos --
    let repos = Git::get_remote_repos()?;
    if repos.len() == 0 {
        bail!("no remote repo specified");
    }

    // -- select the only repo --
    if repos.len() == 1 {
        return Ok(repos[0].clone());
    }

    // -- user select target repo --
    println!("select target repo:");
    repos
        .iter()
        .enumerate()
        .for_each(|(i, x)| println!("[{}] {}", i + 1, x));

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let index = input.trim().parse::<usize>().context("invalid input")?;
    if index > repos.len() {
        bail!("invalid input");
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

    // -- check diff branches --
    if source_branches.len() == target_branches.len()
        && source_branches
            .iter()
            .all(|x| target_branches.iter().any(|y| x == y))
    {
        Echo::success("no branch to sync");
        return;
    }

    // -- remove redundant branches --
    if let SyncStrategy::Override = strategy {
        let redundant_branches = target_branches
            .iter()
            .filter(|x| !source_branches.iter().any(|y| x.as_str() == y))
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        if redundant_branches.len() == 0 {
            Echo::success("no redundant branches");
        } else {
            let finish = Echo::progress("remove redundant branches");
            for branch in &redundant_branches {
                if let Err(err) = del_branch(target, repo, branch) {
                    finish();
                    println!();
                    Echo::error(err.to_string());
                    return;
                };
            }
            finish();
            Echo::success(format!(
                "\rremove redundant branches: {}",
                &redundant_branches.join(", ")
            ));
        }
    }

    // -- create missing branches --
    let missing_branches = source_branches
        .iter()
        .filter(|x| !target_branches.iter().any(|y| x.as_str() == y))
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    if missing_branches.len() == 0 {
        Echo::success("no missing branches");
        return;
    }

    let finish = Echo::progress("create missing branches");
    for branch in &missing_branches {
        if let Err(err) = create_branch(target, repo, branch) {
            finish();
            println!();
            Echo::error(err.to_string());
            return;
        };
    }
    finish();
    Echo::success(format!(
        "\rcreate missing branches: {}",
        &missing_branches.join(", ")
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

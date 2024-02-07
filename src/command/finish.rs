use anyhow::{bail, Result};
use regex::Regex;

use crate::{
    config::definition::{BranchType, Strategy, TargetBranch},
    echo::Echo,
    git::Git,
    utils::run_hook,
};

pub fn finish_task(branch_name: String, branch_type: BranchType) {
    // -- validate branches --
    let branches = match Git::get_local_branches() {
        Err(err) => {
            Echo::error(err.to_string());
            return;
        }
        Ok(branches_v) => branches_v,
    };
    if branches.iter().all(|x| x.as_str() != branch_name) {
        Echo::error(format!("branch {} is not found", branch_name));
        return;
    }

    // -- run before finish hook --
    if run_hook(branch_type.before_finish.clone()).is_err() {
        return;
    }

    // -- collect target branches --
    let mut target_branches = Vec::<TargetBranch>::new();
    branches.iter().for_each(|x| {
        for y in branch_type.to.iter() {
            let regex = Regex::new(&y.name).unwrap();
            if regex.is_match(x) {
                target_branches.push(TargetBranch {
                    name: x.to_string(),
                    strategy: y.strategy.clone(),
                });
                break;
            }
        }
    });

    // -- resolve target branches --
    if resolve_target_branches(&branch_name, &target_branches).is_err() {
        return;
    }

    // -- delete branch --
    let finish = Echo::progress(format!("switch to new branch {}", &branch_name));
    match Git::switch(&branch_name) {
        Err(err) => {
            finish(false, &err.to_string());
            return;
        }
        Ok(_) => finish(true, &format!("switch to new branch {}", &branch_name)),
    }

    // -- run after finish hook --
    let _ = run_hook(branch_type.after_finish.clone());
}

fn resolve_target_branches(branch_name: &str, target_branches: &Vec<TargetBranch>) -> Result<()> {
    for x in target_branches.iter() {
        match x.strategy {
            Strategy::Merge => {
                merge(branch_name, &x.name)?;
            }
            Strategy::Rebase => {
                rebase(branch_name, &x.name)?;
            }
            Strategy::CherryPick => {
                cherry_pick(branch_name, &x.name)?;
            }
        }
    }
    Ok(())
}

fn merge(source_branch: &str, target_branch: &str) -> Result<()> {
    let finish = Echo::progress(format!("merge {} into {}", source_branch, target_branch));

    // -- switch --
    let result = Git::switch(target_branch);
    if let Err(err) = result {
        finish(false, &err.to_string());
        bail!("");
    }

    // -- merge --
    let result = Git::merge(source_branch);
    if let Err(err) = result {
        finish(false, &err.to_string());
        bail!("");
    }

    finish(
        true,
        &format!("merge {} into {}", source_branch, target_branch),
    );
    Ok(())
}

fn rebase(source_branch: &str, target_branch: &str) -> Result<()> {
    let finish = Echo::progress(format!("rebase {} onto {}", target_branch, source_branch));

    // -- switch --
    let result = Git::switch(target_branch);
    if let Err(err) = result {
        finish(false, &err.to_string());
        bail!("");
    }

    // -- rebase --
    let result = Git::rebase(source_branch);
    if let Err(err) = result {
        finish(false, &err.to_string());
        bail!("");
    }

    finish(
        true,
        &format!("rebase {} onto {}", target_branch, source_branch),
    );
    Ok(())
}

fn cherry_pick(source_branch: &str, target_branch: &str) -> Result<()> {
    // -- get diff commits --
    let commits = match Git::diff_commits(source_branch, target_branch) {
        Err(err) => {
            Echo::error(err.to_string());
            bail!("");
        }
        Ok(commits_v) => commits_v,
    };
    if commits.len() == 0 {
        Echo::success(&format!("no commits to cherry pick to {}", target_branch));
        return Ok(());
    }
    let msg = if commits.len() == 1 {
        format!("cherry pick commit {} to {}", &commits[0], target_branch)
    } else {
        format!(
            "cherry pick commits {}..{} to {}",
            &commits[0],
            &commits.last().unwrap(),
            target_branch
        )
    };
    let finish = Echo::progress(&msg);

    // -- switch --
    let result = Git::switch(target_branch);
    if let Err(err) = result {
        finish(false, &err.to_string());
        bail!("");
    }

    // -- cherry pick --
    let result = Git::cherry_pick(commits);
    if let Err(err) = result {
        finish(false, &err.to_string());
        bail!("");
    }

    finish(true, &msg);
    Ok(())
}

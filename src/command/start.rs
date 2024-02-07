use crate::{config::definition::BranchType, echo::Echo, git::Git, utils::run_hook};

pub fn start_task(branch_name: String, branch_type: BranchType) {
    // -- validate branches --
    let branches = match Git::get_local_branches() {
        Err(err) => {
            Echo::error(err.to_string());
            return;
        }
        Ok(branches_v) => branches_v,
    };
    if branches.iter().all(|x| x.as_str() != branch_type.from) {
        Echo::error(format!("source branch {} is not found", branch_type.from));
        return;
    }
    if branches.iter().any(|x| x.as_str() == branch_name) {
        Echo::error(format!("branch {} does exist", branch_name));
        return;
    }

    // -- run before start hook --
    if run_hook(branch_type.before_start.clone(), &branch_name).is_err() {
        return;
    }

    // -- create new branch --
    let finish = Echo::progress(format!("create new branch {}", &branch_name));
    match Git::create_local_branch(&branch_type.from, &branch_name) {
        Err(err) => {
            finish(false, &err.to_string());
            return;
        }
        Ok(_) => finish(true, &format!("create new branch {}", &branch_name)),
    }

    // -- switch to new branch --
    let finish = Echo::progress(format!("switch to new branch {}", &branch_name));
    match Git::switch(&branch_name) {
        Err(err) => {
            finish(false, &err.to_string());
            return;
        }
        Ok(_) => finish(true, &format!("switch to new branch {}", &branch_name)),
    }

    // -- run after start hook --
    let _ = run_hook(branch_type.after_start.clone(), &branch_name);
}

use crate::{config::definition::BranchType, echo::Echo, git::Git, utils::run_hook};

pub fn drop_task(branch_name: String, branch_type: BranchType) {
    // -- validate branches --
    let branches = match Git::get_local_branches() {
        Err(err) => {
            Echo::error(err.to_string());
            return;
        }
        Ok(branches_v) => branches_v,
    };
    if branches.iter().all(|x| x.as_str() != branch_name) {
        Echo::error(format!("target branch {} not found", branch_name));
        return;
    }

    // -- run before drop hook --
    if run_hook(branch_type.before_drop.clone(), &branch_name, &branch_type).is_err() {
        return;
    }

    // -- switch to source branch --
    let finish = Echo::progress(format!("switch to branch {}", &branch_type.from));
    match Git::switch(&branch_type.from) {
        Err(err) => {
            finish(false, &err.to_string());
            return;
        }
        Ok(_) => finish(true, &format!("switch to branch {}", &branch_type.from)),
    }

    // -- delete branch --
    let finish = Echo::progress(format!("delete branch {}", &branch_name));
    match Git::del_local_branch(&branch_name) {
        Err(err) => {
            finish(false, &err.to_string());
            return;
        }
        Ok(_) => finish(true, &format!("delete branch {}", &branch_name)),
    }

    // -- run after drop hook --
    let _ = run_hook(branch_type.after_drop.clone(), &branch_name, &branch_type);
}

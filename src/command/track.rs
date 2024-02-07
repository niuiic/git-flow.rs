use crate::{config::definition::BranchType, echo::Echo, git::Git};

pub fn track_task(branch_name: String, branch_type: BranchType) {
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

    // -- get diff commits --
    let commits = match Git::diff_commits(&branch_name, &branch_type.from) {
        Err(err) => {
            Echo::error(err.to_string());
            return;
        }
        Ok(commits_v) => commits_v,
    };

    if commits.is_empty() {
        Echo::info(&format!(
            "no commits ahead of the source branch {} on {}",
            &branch_type.from, &branch_name,
        ));
    } else {
        Echo::info(&format!(
            "these commits are ahead of the source branch {}:\n",
            &branch_type.from,
        ));
        if let Err(err) = Git::diff_logs(&branch_name, &branch_type.from) {
            Echo::error(err.to_string());
        };
    }
}

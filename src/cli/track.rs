use super::Cli;
use crate::{config::definition::Config, echo::Echo, git::Git};

impl Cli {
    pub fn track(config_list: &Vec<Config>, branch_type: &str, branch_name: &str) {
        let config = config_list
            .iter()
            .find(|x| x.branch_type == branch_type)
            .unwrap();

        let branch_name = config.branch_name.replace("{new_branch}", branch_name);

        let branches = Git::get_local_branches().unwrap();
        if branches
            .iter()
            .find(|x| x.as_str() == branch_name)
            .is_none()
        {
            Echo::error(&format!("Target branch {} not found", branch_name));
            return;
        }

        let result = Git::diff_commits(&branch_name, &config.source_branch).unwrap();
        if result.is_empty() {
            Echo::info(&format!(
                "No commits ahead of the source branch {} on {}",
                config.source_branch, &branch_name,
            ));
            return;
        }
        Echo::info(&format!(
            "These commits are ahead of the source branch {}:\n",
            config.source_branch,
        ));
        if let Err(err) = Git::diff_logs(&branch_name, &config.source_branch) {
            Echo::error(&err.to_string());
        };
    }
}

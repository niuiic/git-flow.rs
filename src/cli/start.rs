use super::Cli;
use crate::{config::definition::Config, echo::Echo, git::Git};

impl Cli {
    pub fn start(config_list: &Vec<Config>, branch_type: &str, branch_name: &str) {
        let config = config_list
            .iter()
            .find(|x| x.branch_type == branch_type)
            .unwrap();

        let branch_name = config.branch_name.replace("{new_branch}", branch_name);

        let branches = Git::get_branches().unwrap();
        if branches
            .iter()
            .find(|x| x.as_str() == config.source_branch)
            .is_none()
        {
            Echo::error(&format!("Source branch {} not found", config.source_branch));
            return;
        }
        if branches
            .iter()
            .find(|x| x.as_str() == branch_name)
            .is_some()
        {
            Echo::error(&format!("Branch {} exists", branch_name));
            return;
        }

        Echo::progress(&format!("Create new branch {}", &branch_name));
        if let Err(err) = Git::create_branch(&config.source_branch, &branch_name) {
            println!();
            Echo::error(&err.to_string());
            return;
        };
        print!("\r");
        Echo::success(&format!("Create new branch {}", &branch_name));

        Echo::progress(&format!("Switch to branch {}", &branch_name));
        if let Err(err) = Git::switch(&branch_name) {
            println!();
            Echo::error(&err.to_string());
            return;
        };
        print!("\r");
        Echo::success(&format!("Switch to branch {}", &branch_name));
    }
}

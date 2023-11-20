use super::Cli;
use crate::{config::definition::Config, echo::Echo, git::Git};

impl Cli {
    pub fn drop(config_list: &Vec<Config>, branch_type: &str, branch_name: &str) {
        let config = config_list
            .iter()
            .find(|x| x.branch_type == branch_type)
            .unwrap();

        let branch_name = config.branch_name.replace("{new_branch}", branch_name);

        let branches = Git::get_branches().unwrap();
        if branches
            .iter()
            .find(|x| x.as_str() == branch_name)
            .is_none()
        {
            Echo::error(&format!("Target branch {} not found", branch_name));
            return;
        }

        let stop = Echo::progress(&format!("Switch to branch {}", &config.source_branch));
        let result = Git::switch(&config.source_branch);
        stop();
        if let Err(err) = result {
            println!();
            Echo::error(&err.to_string());
            return;
        };
        print!("\r");
        Echo::success(&format!("Switch to branch {}", &config.source_branch));

        let stop = Echo::progress(&format!("Delete branch {}", &branch_name));
        let result = Git::del_branch(&branch_name);
        stop();
        if let Err(err) = result {
            println!();
            Echo::error(&err.to_string());
            return;
        }
        print!("\r");
        Echo::success(&format!("Delete branch {}", &branch_name));
    }
}

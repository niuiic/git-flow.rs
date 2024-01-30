use super::Cli;
use crate::{
    cli::hook::{exec_hook, Hook},
    config::definition::Config,
    echo::Echo,
    git::Git,
};

impl Cli {
    pub fn start(config_list: &Vec<Config>, branch_type: &str, branch_name: &str) {
        // %% validate branch name %%
        if config_list.iter().any(|x| x.branch_type == branch_name) {
            Echo::error(&format!(
                "Branch name should be different from branch types"
            ));
            return;
        }

        // %% find config %%
        let config = config_list
            .iter()
            .find(|x| x.branch_type == branch_type)
            .unwrap();

        // %% calculate branch name %%
        let full_branch_name = config.branch_name.replace("{new_branch}", branch_name);

        // %% get/validate branches %%
        let branches = Git::get_local_branches().unwrap();
        if branches.iter().all(|x| x.as_str() != config.source_branch) {
            Echo::error(&format!("Source branch {} not found", config.source_branch));
            return;
        }
        if branches.iter().any(|x| x.as_str() == full_branch_name) {
            Echo::error(&format!("Branch {} exists", full_branch_name));
            return;
        }

        // %% run before start hook %%
        if let Err(_) = exec_hook(&config, Hook::BeforeStart, branch_name) {
            return;
        }

        // %% create new branch %%
        let stop = Echo::progress(&format!("Create new branch {}", &full_branch_name));
        let result = Git::create_local_branch(&config.source_branch, &full_branch_name);
        stop();
        if let Err(err) = result {
            println!();
            Echo::error(&err.to_string());
            return;
        };
        print!("\r");
        Echo::success(&format!("Create new branch {}", &full_branch_name));

        // %% switch to new branch %%
        let stop = Echo::progress(&format!("Switch to branch {}", &full_branch_name));
        let result = Git::switch(&full_branch_name);
        stop();
        if let Err(err) = result {
            println!();
            Echo::error(&err.to_string());
            return;
        };
        print!("\r");
        Echo::success(&format!("Switch to branch {}", &full_branch_name));

        // %% run after start hook %%
        if let Err(_) = exec_hook(&config, Hook::AfterStart, branch_name) {
            return;
        }
    }
}

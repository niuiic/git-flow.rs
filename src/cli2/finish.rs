use regex::Regex;

use super::Cli;
use crate::{
    cli::hook::{exec_hook, Hook},
    config::definition::{Config, Strategy, TargetBranch},
    echo::Echo,
    git::Git,
};

impl Cli {
    pub fn finish(config_list: &Vec<Config>, branch_type: &str, branch_name: &str) {
        // %% find config %%
        let config = config_list
            .iter()
            .find(|x| x.branch_type == branch_type)
            .unwrap();

        // %% calculate branch name %%
        let full_branch_name = config.branch_name.replace("{new_branch}", branch_name);

        // %% get/validate branches %%
        let branches = Git::get_local_branches().unwrap();
        if branches.iter().all(|x| x.as_str() != full_branch_name) {
            Echo::error(&format!("Branch {} not found", full_branch_name));
            return;
        }

        // %% collect target branches %%
        let mut target_branches = Vec::<TargetBranch>::new();
        for target in &config.target_branches {
            let regex = Regex::new(&target.name).unwrap();
            branches.iter().filter(|x| regex.is_match(x)).for_each(|x| {
                if target_branches.iter().any(|y| y.name == *x) {
                    return;
                }
                target_branches.push(TargetBranch {
                    name: x.clone(),
                    strategy: target.strategy.clone(),
                });
            });
        }

        // %% run before finish hook %%
        if let Err(_) = exec_hook(&config, Hook::BeforeFinish, branch_name) {
            return;
        }

        // %% resolve target %%
        for target in &target_branches {
            match target.strategy {
                Strategy::Merge => {
                    let stop = Echo::progress(&format!(
                        "Merge {} into {}",
                        &full_branch_name, &target.name
                    ));
                    let result = Git::switch(&target.name);
                    stop();
                    if let Err(err) = result {
                        println!();
                        Echo::error(&err.to_string());
                        return;
                    };
                    if let Err(err) = Git::merge(&full_branch_name) {
                        let err_info = err.to_string();
                        println!();
                        Echo::error(if err_info.is_empty() {
                            "Automatic merge failed.\nFix conflicts and then re-run the command."
                        } else {
                            &err_info
                        });
                        return;
                    };
                    print!("\r");
                    Echo::success(&format!(
                        "Merge {} into {}",
                        &full_branch_name, &target.name
                    ));
                }
                Strategy::Rebase => {
                    let stop = Echo::progress(&format!(
                        "Rebase {} onto {}",
                        &target.name, &full_branch_name
                    ));
                    let result = Git::switch(&target.name);
                    stop();
                    if let Err(err) = result {
                        println!();
                        Echo::error(&err.to_string());
                        return;
                    };
                    if let Err(err) = Git::rebase(&full_branch_name) {
                        let err_info = err.to_string();
                        println!();
                        Echo::error(if err_info.is_empty() {
                            "Automatic rebase failed.\nFix conflicts and then re-run the command."
                        } else {
                            &err_info
                        });
                        return;
                    };
                    print!("\r");
                    Echo::success(&format!(
                        "Rebase {} onto {}",
                        &target.name, &full_branch_name
                    ));
                }
                Strategy::CherryPick => {
                    let commits = Git::diff_commits(&full_branch_name, &target.name).unwrap();
                    if commits.len() == 0 {
                        Echo::warn(&format!("No commits to cherry pick to {}", &target.name));
                        continue;
                    }

                    let info = if commits.len() == 1 {
                        format!("Cherry pick commit {} to {}", &commits[0], &target.name)
                    } else {
                        format!(
                            "Cherry pick commits {}..{} to {}",
                            &commits[0],
                            &commits.last().unwrap(),
                            &target.name
                        )
                    };

                    let stop = Echo::progress(&info);
                    let result = Git::switch(&target.name);
                    stop();
                    if let Err(err) = result {
                        println!();
                        Echo::error(&err.to_string());
                        return;
                    };
                    if let Err(err) = Git::cherry_pick(commits) {
                        println!();
                        Echo::error(&err.to_string());
                        return;
                    };
                    print!("\r");
                    Echo::success(&info);
                }
            }
        }

        // %% delete branch %%
        let stop = Echo::progress(&format!("Delete branch {}", &full_branch_name));
        let result = Git::switch(&config.source_branch);
        stop();
        if let Err(err) = result {
            println!();
            Echo::error(&err.to_string());
            return;
        };
        if let Err(err) = Git::del_local_branch(&full_branch_name) {
            println!();
            Echo::error(&err.to_string());
            return;
        };
        print!("\r");
        Echo::success(&format!("Delete branch {}", &full_branch_name));

        // %% run after finish hook %%
        if let Err(_) = exec_hook(&config, Hook::AfterFinish, branch_name) {
            return;
        }
    }
}

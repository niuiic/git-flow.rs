use regex::Regex;

use super::Cli;
use crate::{
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
        let branch_name = config.branch_name.replace("{new_branch}", branch_name);

        // %% get/validate branches %%
        let branches = Git::get_branches().unwrap();
        if branches.iter().all(|x| x.as_str() != branch_name) {
            Echo::error(&format!("Branch {} not found", branch_name));
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

        // %% resolve target %%
        for target in &target_branches {
            match target.strategy {
                Strategy::Merge => {
                    Echo::progress(&format!("Merge {} into {}", &branch_name, &target.name));
                    if let Err(err) = Git::switch(&target.name) {
                        println!();
                        Echo::error(&err.to_string());
                        return;
                    };
                    if let Err(err) = Git::merge(&branch_name) {
                        println!();
                        let err_info = err.to_string();
                        Echo::error(if err_info.is_empty() {
                            "Automatic merge failed.\nFix conflicts and then re-run the command."
                        } else {
                            &err_info
                        });
                        return;
                    };
                    print!("\r");
                    Echo::success(&format!("Merge {} into {}", &branch_name, &target.name));
                }
                Strategy::Rebase => {
                    Echo::progress(&format!("Rebase {} onto {}", &target.name, &branch_name));
                    if let Err(err) = Git::switch(&target.name) {
                        println!();
                        Echo::error(&err.to_string());
                        return;
                    };
                    if let Err(err) = Git::rebase(&branch_name) {
                        println!();
                        let err_info = err.to_string();
                        Echo::error(if err_info.is_empty() {
                            "Automatic rebase failed.\nFix conflicts and then re-run the command."
                        } else {
                            &err_info
                        });
                        return;
                    };
                    print!("\r");
                    Echo::success(&format!("Rebase {} onto {}", &target.name, &branch_name));
                }
                Strategy::CherryPick => {
                    let commits = Git::diff_commits(&branch_name, &target.name).unwrap();
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

                    Echo::progress(&info);
                    if let Err(err) = Git::switch(&target.name) {
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
        Echo::progress(&format!("Delete branch {}", &branch_name));
        if let Err(err) = Git::switch(&config.source_branch) {
            println!();
            Echo::error(&err.to_string());
            return;
        };
        if let Err(err) = Git::del_branch(&branch_name) {
            println!();
            Echo::error(&err.to_string());
            return;
        };
        print!("\r");
        Echo::success(&format!("Delete branch {}", &branch_name));
    }
}

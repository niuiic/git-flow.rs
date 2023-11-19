use super::Cli;
use crate::{
    config::definition::{Config, Strategy},
    echo::Echo,
    git::Git,
};

impl Cli {
    pub fn finish(config_list: &Vec<Config>, branch_type: &str, branch_name: &str) {
        let config = config_list
            .iter()
            .find(|x| x.branch_type == branch_type)
            .unwrap();

        let branch_name = config.branch_name.replace("{new_branch}", branch_name);

        let branches = Git::get_branches().unwrap();
        let mut work_branches: Vec<String> = config
            .target_branches
            .iter()
            .map(|x| x.name.to_string())
            .collect();
        work_branches.push(config.source_branch.clone());
        work_branches.push(branch_name.clone());
        let missing_branches: Vec<String> = work_branches
            .iter()
            .filter(|x| branches.iter().find(|y| y.as_str() == x.as_str()).is_none())
            .map(|x| x.to_string())
            .collect();
        if missing_branches.len() > 0 {
            Echo::error(&format!("Branch {} not found.", missing_branches.join(",")));
            return;
        }

        for target in &config.target_branches {
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

use crate::{built_info, config::definition::Config, echo::Echo, git::Git};

pub struct Cli {}

#[cfg(test)]
mod test;

impl Cli {
    pub fn help(config_list: &Vec<Config>) {
        println!("Extensible git flow written in rust.\n");
        println!("Usage: git-flow <command>\n");
        println!("Avaliable commands:\n");
        println!("-h, --help\n\tPrint help");
        println!("-v, --version\n\tPrint version");
        println!("start [<branch_type> <branch_name>]/[<full_branch_name>]\n\tstart a task");
        println!("finish [<branch_type> <branch_name>]/[<full_branch_name>]\n\tfinish a task");
        println!("drop [<branch_type> <branch_name>]/[<full_branch_name>]\n\tgive up a task");
        println!("track [<branch_type> <branch_name>]/[<full_branch_name>]\n\ttrack a task");
        println!("continue\n\tContinue unfinished task");
        if config_list.len() > 0 {
            println!("\nConfigured branch types:\n");
            config_list.iter().for_each(|config| {
                println!(
                    "{}\n\tfrom {} to {}",
                    config.branch_type,
                    config.source_branch,
                    config
                        .target_branches
                        .iter()
                        .map(|x| x.name.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                );
            });
        } else {
            Echo::warn("No configured branch type found");
        }
    }

    pub fn version() {
        println!("git-flow version {}", built_info::PKG_VERSION);
    }

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

        Git::create_branch(&config.source_branch, &branch_name).unwrap();
        Echo::success(&format!("Create new branch {}", &branch_name));
        Git::switch(&branch_name).unwrap();
        Echo::success(&format!("Switch to branch {}", &branch_name));
    }

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
        Git::switch(&config.source_branch).unwrap();
        Echo::success(&format!("Switch to branch {}", &config.source_branch));
        Git::del_branch(&branch_name).unwrap();
        Echo::success(&format!("Delete branch {}", &branch_name));
    }

    pub fn track(config_list: &Vec<Config>, branch_type: &str, branch_name: &str) {
        let config = config_list
            .iter()
            .find(|x| x.branch_type == branch_type)
            .unwrap();

        let branch_name = config.branch_name.replace("{new_branch}", branch_name);
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
        Git::diff_logs(&branch_name, &config.source_branch).unwrap();
    }
}

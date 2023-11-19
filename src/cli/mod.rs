use crate::{built_info, config::definition::Config, echo::Echo};

#[cfg(test)]
mod test;

mod drop;
mod finish;
mod start;
mod track;

pub struct Cli {}

impl Cli {
    pub fn version() {
        println!("git-flow version {}", built_info::PKG_VERSION);
    }

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
}

mod built_info;
mod cli;
mod config;
mod echo;
mod git;

use std::env;

use cli::Cli;
use config::{parse::get_branch_type_name, read::read_config, validate::validate_config};
use echo::Echo;
use git::Git;

fn main() {
    if !Git::has_git() {
        Echo::error("git not installed");
        return;
    }

    let config_list = read_config().unwrap();
    validate_config(&config_list).unwrap();

    let args = &env::args().collect::<Vec<String>>()[1..];

    match args[0].as_str() {
        "-h" | "--help" => Cli::help(&config_list),
        "-v" | "--version" => Cli::version(),
        "start" => {
            if args.len() == 1 {
                Echo::error("No enough arguments");
                return;
            }
            let (branch_type, branch_name) = get_branch_type_name(
                &config_list,
                &args[1..].iter().map(|x| x.to_string()).collect(),
            )
            .unwrap();
            Cli::start(&config_list, &branch_type, &branch_name);
        }
        "drop" => {
            if args.len() == 1 {
                Echo::error("No enough arguments");
                return;
            }
            let (branch_type, branch_name) = get_branch_type_name(
                &config_list,
                &args[1..].iter().map(|x| x.to_string()).collect(),
            )
            .unwrap();
            Cli::drop(&config_list, &branch_type, &branch_name);
        }
        _ => Cli::help(&config_list),
    }
}

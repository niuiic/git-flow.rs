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

    if args.get(0).is_none() {
        Cli::help(&config_list);
        return;
    }

    match args[0].as_str() {
        "-h" | "--help" => Cli::help(&config_list),
        "-v" | "--version" => Cli::version(),
        "start" | "finish" | "drop" | "track" => {
            if args.len() == 1 {
                Echo::error("No enough arguments");
                return;
            }

            match get_branch_type_name(
                &config_list,
                &args[1..].iter().map(|x| x.to_string()).collect(),
            ) {
                Ok((branch_type, branch_name)) => {
                    if args[0].as_str() == "start" {
                        Cli::start(&config_list, &branch_type, &branch_name);
                    } else if args[0].as_str() == "finish" {
                        Cli::finish(&config_list, &branch_type, &branch_name);
                    } else if args[0].as_str() == "drop" {
                        Cli::drop(&config_list, &branch_type, &branch_name);
                    } else if args[0].as_str() == "track" {
                        Cli::track(&config_list, &branch_type, &branch_name);
                    }
                }
                Err(err) => {
                    Echo::error(&err.to_string());
                }
            }
        }
        _ => Cli::help(&config_list),
    }
}

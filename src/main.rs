use clap::Parser;
use cli::{Args, Command};

mod cli;
mod command;
mod config;
mod echo;
mod git;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match &args.command {
        Command::List => command::list::list_branch_types(args.config.clone()),
        Command::Check => command::check::check_config(args.config.clone()),
        _ => {}
    }

    // if !Git::has_git() {
    //     Echo::error("git not installed");
    //     return;
    // }
    // if !Git::in_git_project() {
    //     Echo::error("You are not in a git project");
    //     return;
    // }

    // // %% read config %%
    // let config_list = match read_config() {
    //     Err(err) => {
    //         Echo::error(&err.to_string());
    //         return;
    //     }
    //     Ok(value) => value,
    // };
    // if let Err(err) = validate_config(&config_list) {
    //     Echo::error(&err.to_string());
    //     return;
    // };

    // // %% receive args %%
    // let args = &env::args().collect::<Vec<String>>()[1..];
    // if args.get(0).is_none() {
    //     Cli::help(&config_list);
    //     return;
    // }

    // // %% exec commands %%
    // match args[0].as_str() {
    //     "-h" | "--help" => Cli::help(&config_list),
    //     "-v" | "--version" => Cli::version(),
    //     "start" | "finish" | "drop" | "track" => {
    //         if args.len() == 1 {
    //             Echo::error("No enough arguments");
    //             return;
    //         }

    //         match get_branch_type_name(
    //             &config_list,
    //             &args[1..].iter().map(|x| x.to_string()).collect(),
    //         ) {
    //             Ok((branch_type, branch_name)) => {
    //                 if args[0].as_str() == "start" {
    //                     Cli::start(&config_list, &branch_type, &branch_name);
    //                 } else if args[0].as_str() == "finish" {
    //                     Cli::finish(&config_list, &branch_type, &branch_name);
    //                 } else if args[0].as_str() == "drop" {
    //                     Cli::drop(&config_list, &branch_type, &branch_name);
    //                 } else if args[0].as_str() == "track" {
    //                     Cli::track(&config_list, &branch_type, &branch_name);
    //                 }
    //             }
    //             Err(err) => {
    //                 Echo::error(&err.to_string());
    //             }
    //         }
    //     }
    //     "sync" => {
    //         let target = match args.get(1) {
    //             Some(value) => match value.as_str() {
    //                 "remote" => SyncTarget::Remote,
    //                 "local" => SyncTarget::Local,
    //                 _ => {
    //                     Echo::error("Wrong arguments");
    //                     return;
    //                 }
    //             },
    //             _ => {
    //                 Echo::error("Wrong arguments");
    //                 return;
    //             }
    //         };

    //         let strategy = if args[2..].iter().any(|x| x.as_str() == "--override") {
    //             SyncStrategy::Override
    //         } else {
    //             SyncStrategy::Increment
    //         };

    //         Cli::sync(target, strategy);
    //     }
    //     _ => Cli::help(&config_list),
    // }
}

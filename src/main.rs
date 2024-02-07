use clap::Parser;
use cli::{Args, Command};
use echo::Echo;
use utils::{env_valid, get_branch_type_name};

mod cli;
mod command;
mod config;
mod echo;
mod git;
mod utils;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match &args.command {
        Command::List => command::list::list_branch_types(args.config.clone()),
        Command::Check => command::check::check_config(args.config.clone()),
        Command::Sync { target, strategy } => {
            if !env_valid() {
                return;
            }

            command::sync::sync_repo_branches(
                target.clone(),
                strategy.clone().unwrap_or(cli::SyncStrategy::Increment),
            );
        }
        Command::Start {
            branch_name,
            branch_type,
        } => {
            if !env_valid() {
                return;
            }

            match get_branch_type_name(branch_name.clone(), branch_type.clone(), args.config) {
                Err(err) => Echo::error(err.to_string()),
                Ok((branch_name, branch_type)) => {
                    command::start::start_task(branch_name, branch_type);
                }
            }
        }
        Command::Finish {
            branch_name,
            branch_type,
        } => {
            if !env_valid() {
                return;
            }

            match get_branch_type_name(branch_name.clone(), branch_type.clone(), args.config) {
                Err(err) => Echo::error(err.to_string()),
                Ok((branch_name, branch_type)) => {
                    command::finish::finish_task(branch_name, branch_type);
                }
            }
        }
        Command::Drop {
            branch_name,
            branch_type,
        } => {
            if !env_valid() {
                return;
            }

            match get_branch_type_name(branch_name.clone(), branch_type.clone(), args.config) {
                Err(err) => Echo::error(err.to_string()),
                Ok((branch_name, branch_type)) => {
                    command::drop::drop_task(branch_name, branch_type);
                }
            }
        }
        _ => {}
    }
}

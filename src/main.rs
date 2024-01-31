use clap::Parser;
use cli::{Args, Command};
use echo::Echo;
use git::Git;

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
        Command::Sync { target, strategy } => {
            if !env_valid() {
                return;
            }

            command::sync::sync_repo_branches(
                target.clone(),
                strategy.clone().unwrap_or(cli::SyncStrategy::Increment),
            );
        }
        _ => {}
    }
}

fn env_valid() -> bool {
    if !Git::git_installed() {
        Echo::error("git command is not found");
        return false;
    }

    if !Git::in_git_project() {
        Echo::error("not in a git project");
        return false;
    }

    true
}

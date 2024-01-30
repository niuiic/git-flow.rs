use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[clap(name = "git-flow", version)]
pub struct Args {
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// start a task
    Start {
        branch_type: String,
        branch_name: String,
    },
    /// finish a task
    Finish {
        branch_type: String,
        branch_name: String,
    },
    /// drop a task
    Drop {
        branch_type: String,
        branch_name: String,
    },
    /// track a task
    Track {
        branch_type: String,
        branch_name: String,
    },
    /// sync branches
    Sync {
        target: SyncTarget,
        strategy: Option<SyncStrategy>,
    },
    /// list avaliable branch types
    List,
    /// check config
    Check,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SyncTarget {
    Local,
    Remote,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SyncStrategy {
    Override,
    Increment,
}

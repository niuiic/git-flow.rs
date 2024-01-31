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
        /// input full branch name if no branch type input
        branch_name: String,
        branch_type: Option<String>,
    },
    /// finish a task
    Finish {
        /// input full branch name if no branch type input
        branch_name: String,
        branch_type: Option<String>,
    },
    /// drop a task
    Drop {
        /// input full branch name if no branch type input
        branch_name: String,
        branch_type: Option<String>,
    },
    /// track a task
    Track {
        /// input full branch name if no branch type input
        branch_name: String,
        branch_type: Option<String>,
    },
    /// sync branches
    Sync {
        target: SyncTarget,
        /// default is increment
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

use super::{
    sync::{SyncStrategy, SyncTarget},
    *,
};
use crate::config::read::read_config;

#[test]
fn help_t() {
    let config_list = read_config().unwrap();
    Cli::help(&config_list);
}

#[test]
fn version_t() {
    Cli::version();
}

#[tokio::test]
async fn start_t() {
    let config_list = read_config().unwrap();
    Cli::start(&config_list, "feature", "test");
}

#[test]
fn drop_t() {
    let config_list = read_config().unwrap();
    Cli::drop(&config_list, "feature", "test");
}

#[tokio::test]
async fn sync_t() {
    Cli::sync(SyncTarget::Local, SyncStrategy::Increment)
}

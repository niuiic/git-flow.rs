use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub timeout: Option<u64>,
    pub branch_types: Vec<BranchType>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BranchType {
    pub name: String,
    pub create: String,
    pub from: String,
    pub to: Vec<TargetBranch>,
    pub before_start: Option<Command>,
    pub after_start: Option<Command>,
    pub before_finish: Option<Command>,
    pub after_finish: Option<Command>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TargetBranch {
    pub name: String,
    pub strategy: Strategy,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Strategy {
    #[serde(rename = "merge")]
    Merge,
    #[serde(rename = "rebase")]
    Rebase,
    #[serde(rename = "cherry-pick")]
    CherryPick,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Command {
    pub command: String,
    pub args: Vec<String>,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(rename = "type")]
    pub branch_type: String,
    #[serde(rename = "name")]
    pub branch_name: String,
    #[serde(rename = "from")]
    pub source_branch: String,
    #[serde(rename = "to")]
    pub target_branches: Vec<TargetBranch>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TargetBranch {
    #[serde(rename = "branch")]
    pub name: String,
    pub strategy: Strategy,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Strategy {
    #[serde(rename = "merge")]
    Merge,
    #[serde(rename = "rebase")]
    Rebase,
    #[serde(rename = "check-pick")]
    CherryPick,
}

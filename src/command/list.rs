use std::path::PathBuf;

use tabled::{Table, Tabled};

use crate::{config::read::read_config, echo::Echo};

#[derive(Tabled)]
struct BranchType {
    name: String,
    create: String,
    from: String,
    to: String,
}

pub fn list_branch_types(config_path: Option<PathBuf>) {
    let config = read_config(config_path);

    match config {
        Err(err) => {
            Echo::error(err.to_string());
        }
        Ok(config_v) => {
            if config_v.branch_types.len() == 0 {
                Echo::warning("no branch types avaliable");
                return;
            }

            let branch_types: Vec<BranchType> = config_v
                .branch_types
                .iter()
                .map(|x| BranchType {
                    name: x.name.clone(),
                    create: x.create.clone(),
                    from: x.from.clone(),
                    to: x
                        .to
                        .iter()
                        .map(|y| y.name.clone())
                        .collect::<Vec<String>>()
                        .join(", "),
                })
                .collect();
            println!("{}", Table::new(branch_types).to_string())
        }
    }
}

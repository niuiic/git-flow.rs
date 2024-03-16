use std::path::PathBuf;

use anyhow::{Context, Result};
use tabled::{
    settings::{peaker::PriorityMax, Width},
    Table, Tabled,
};
use terminal_size::{terminal_size, Height as TerminalHeight, Width as TerminalWidth};

use crate::{
    config::{definition::Command, read::read_config},
    echo::Echo,
};

#[derive(Tabled)]
struct BranchType {
    name: String,
    create: String,
    from: String,
    to: String,
    before_start: String,
    after_start: String,
    before_finish: String,
    after_finish: String,
    before_drop: String,
    after_drop: String,
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
                    before_start: command_to_string(x.before_start.clone()),
                    after_start: command_to_string(x.after_start.clone()),
                    before_finish: command_to_string(x.before_finish.clone()),
                    after_finish: command_to_string(x.after_finish.clone()),
                    before_drop: command_to_string(x.before_drop.clone()),
                    after_drop: command_to_string(x.after_drop.clone()),
                })
                .collect();

            let width = match get_terminal_size() {
                Err(_) => {
                    Echo::error("unable to get terminal size");
                    return;
                }
                Ok(size) => size.0,
            };
            println!(
                "{}",
                Table::new(branch_types)
                    .with(Width::wrap(width).priority::<PriorityMax>())
                    .with(Width::increase(width))
                    .to_string()
            )
        }
    }
}

fn command_to_string(command: Option<Command>) -> String {
    match command {
        None => String::new(),
        Some(command_v) => format!("{} {}", command_v.command, command_v.args.join(" ")),
    }
}

fn get_terminal_size() -> Result<(usize, usize)> {
    let (TerminalWidth(width), TerminalHeight(height)) = terminal_size().context("unable")?;

    Ok((width as usize, height as usize))
}

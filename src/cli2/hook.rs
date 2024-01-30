use std::process::Command;

use anyhow::{bail, Result};

use crate::{config::definition::Config, echo::Echo};

pub enum Hook {
    BeforeStart,
    AfterStart,
    BeforeFinish,
    AfterFinish,
}

pub fn exec_hook(config: &Config, hook: Hook, branch: &str) -> Result<()> {
    let hooks = match &config.hooks {
        None => return Ok(()),
        Some(value) => value,
    };

    let work_hook = match hook {
        Hook::BeforeStart => &hooks.before_start,
        Hook::AfterStart => &hooks.after_start,
        Hook::BeforeFinish => &hooks.before_finish,
        Hook::AfterFinish => &hooks.after_finish,
    };

    let task = match &work_hook {
        None => return Ok(()),
        Some(value) => value,
    };

    let args: Vec<String> = task
        .args
        .iter()
        .map(|x| x.replace("{new_branch}", branch))
        .collect();

    let msg = match hook {
        Hook::BeforeStart => format!("Run before start hook: {} {}", task.command, args.join(" ")),
        Hook::AfterStart => format!("Run after start hook: {} {}", task.command, args.join(" ")),
        Hook::BeforeFinish => format!(
            "Run before finish hook: {} {}",
            task.command,
            args.join(" ")
        ),
        Hook::AfterFinish => format!("Run after finish hook: {} {}", task.command, args.join(" ")),
    };

    let stop = Echo::progress(&msg);

    let output = Command::new(task.command.clone()).args(args).output();
    stop();
    let output = match output {
        Err(err) => {
            println!();
            Echo::error(&err.to_string());
            bail!("");
        }
        Ok(value) => value,
    };
    if output.status.success() {
        print!("\r");
        Echo::success(&msg);
        Ok(())
    } else {
        println!();
        Echo::error(&String::from_utf8(output.stderr).unwrap());
        bail!("");
    }
}

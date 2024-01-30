use std::{
    io::{self, Write},
    process::Command,
};

use anyhow::{bail, Result};

#[cfg(test)]
mod test;

mod branch;

pub struct Git {}

// # status
impl Git {
    pub fn git_installed() -> bool {
        let output = Command::new("git").arg("--version").output();
        output.is_ok()
    }

    pub fn in_git_project() -> bool {
        let output = Command::new("git")
            .args(["rev-parse", "--is-inside-work-tree"])
            .output();
        if let Ok(output) = output {
            output.status.success()
        } else {
            false
        }
    }
}

// # combine
impl Git {
    pub fn merge(source_branch: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["merge", source_branch])
            .output()?;
        if output.status.success() {
            Ok(())
        } else {
            bail!(String::from_utf8(output.stderr).unwrap());
        }
    }

    pub fn rebase(base_branch: &str) -> Result<()> {
        let output = Command::new("git").args(["rebase", base_branch]).output()?;
        if output.status.success() {
            Ok(())
        } else {
            bail!(String::from_utf8(output.stderr).unwrap());
        }
    }

    pub fn cherry_pick(commits: Vec<String>) -> Result<()> {
        let output = Command::new("git")
            .arg("cherry-pick")
            .args(commits)
            .output()?;
        if output.status.success() {
            Ok(())
        } else {
            bail!(String::from_utf8(output.stderr).unwrap());
        }
    }
}

// # other
impl Git {
    pub fn switch(target_branch: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["switch", target_branch])
            .output()?;
        if output.status.success() {
            Ok(())
        } else {
            bail!(String::from_utf8(output.stderr).unwrap());
        }
    }

    /// commits on source_branch but not on target_branch
    pub fn diff_commits(source_branch: &str, target_branch: &str) -> Result<Vec<String>> {
        let output = Command::new("git")
            .args([
                "log",
                "--format=%H",
                &format!("{}..{}", target_branch, source_branch),
            ])
            .output()?;
        if !output.status.success() {
            bail!(String::from_utf8(output.stderr).unwrap());
        }

        let output_str = String::from_utf8(output.stdout).unwrap();
        Ok(output_str
            .split('\n')
            .filter(|x| *x != "")
            .map(|x| x.to_string())
            .collect::<Vec<String>>())
    }

    /// output commits on source_branch but not on target_branch
    pub fn diff_logs(source_branch: &str, target_branch: &str) -> Result<()> {
        let output = Command::new("git")
            .args([
                "log",
                "--color",
                &format!("{}..{}", target_branch, source_branch),
            ])
            .output()?;
        if !output.status.success() {
            bail!(String::from_utf8(output.stderr).unwrap());
        }

        io::stdout().write_all(&output.stdout)?;
        Ok(())
    }

    pub fn get_remote_repos() -> Result<Vec<String>> {
        let output = Command::new("git").args(["remote"]).output()?;
        if !output.status.success() {
            bail!(String::from_utf8(output.stderr).unwrap());
        }

        let output_str = String::from_utf8(output.stdout).unwrap();
        Ok(output_str
            .split('\n')
            .filter(|x| *x != "")
            .map(|x| x.to_string())
            .collect::<Vec<String>>())
    }
}

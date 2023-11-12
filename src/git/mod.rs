use std::process::Command;

use anyhow::{bail, Ok, Result};

#[cfg(test)]
mod test;

pub struct Git {}

impl Git {
    pub fn has_git() -> bool {
        let output = Command::new("git").arg("--version").output();
        if output.is_ok() {
            true
        } else {
            false
        }
    }

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

    pub fn del_branch(target_branch: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["branch", "-d", target_branch])
            .output()?;
        if output.status.success() {
            Ok(())
        } else {
            bail!(String::from_utf8(output.stderr).unwrap());
        }
    }

    pub fn create_branch(source_branch: &str, target_branch: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["branch", target_branch, source_branch])
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

    pub fn get_branches() -> Result<Vec<String>> {
        let output = Command::new("git").args(["branch"]).output()?;
        if !output.status.success() {
            bail!(String::from_utf8(output.stderr).unwrap());
        }

        let output_str = String::from_utf8(output.stdout).unwrap();
        Ok(output_str
            .split('\n')
            .filter(|x| *x != "")
            .map(|x| x.replace("*", "").trim().to_string())
            .collect::<Vec<String>>())
    }
}

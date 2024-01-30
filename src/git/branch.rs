use std::process::Command;

use anyhow::{bail, Result};
use regex::Regex;

use super::Git;

// # delete
impl Git {
    pub fn del_local_branch(target_branch: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["branch", "-D", target_branch])
            .output()?;
        if output.status.success() {
            Ok(())
        } else {
            bail!(String::from_utf8(output.stderr).unwrap());
        }
    }

    pub fn del_remote_branch(target_repo: &str, target_branch: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["push", target_repo, "--delete", target_branch])
            .output()?;
        if output.status.success() {
            Ok(())
        } else {
            bail!(String::from_utf8(output.stderr).unwrap());
        }
    }
}

// # create
impl Git {
    pub fn create_local_branch(source_branch: &str, target_branch: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["branch", target_branch, source_branch])
            .output()?;
        if output.status.success() {
            Ok(())
        } else {
            bail!(String::from_utf8(output.stderr).unwrap());
        }
    }

    /// Create remote branch from local branch
    pub fn create_remote_branch(repo: &str, local_branch: &str, remote_branch: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["push", repo, &format!("{}:{}", local_branch, remote_branch)])
            .output()?;
        if output.status.success() {
            Ok(())
        } else {
            bail!(String::from_utf8(output.stderr).unwrap());
        }
    }
}

// # get
impl Git {
    pub fn fetch_remote_data() -> Result<()> {
        let output = Command::new("git").args(["fetch", "--all"]).output()?;
        if !output.status.success() {
            bail!(String::from_utf8(output.stderr).unwrap());
        }
        Ok(())
    }

    pub fn get_local_branches() -> Result<Vec<String>> {
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

    pub fn get_remote_branches(repo: &str) -> Result<Vec<String>> {
        let output = Command::new("git").args(["branch", "-r"]).output()?;
        if !output.status.success() {
            bail!(String::from_utf8(output.stderr).unwrap());
        }

        let output_str = String::from_utf8(output.stdout).unwrap();
        let lines = output_str
            .split('\n')
            .enumerate()
            .filter(|(i, x)| *i > 0 && *x != "")
            .map(|(_, x)| x);

        let branch_regex = Regex::new(r"(\S+/\S+)").unwrap();
        let branches = lines
            .map(|x| {
                if let Some(captures) = branch_regex.captures(x) {
                    if let Some(matched) = captures.get(1) {
                        return Some(matched.as_str());
                    }
                }

                None
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap());

        let branch_name_regex = Regex::new(&format!(r"{}/(\S+)", repo)).unwrap();
        let branch_names = branches
            .map(|x| {
                if let Some(captures) = branch_name_regex.captures(x) {
                    if let Some(matched) = captures.get(1) {
                        return Some(matched.as_str());
                    }
                }

                None
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap().to_string());

        Ok(branch_names.collect::<Vec<String>>())
    }
}

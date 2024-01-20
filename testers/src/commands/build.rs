use std::process::{Command, ExitStatus};
use anyhow::Result;


pub fn build(contest_dir: String) -> Result<ExitStatus> {
    let status = Command::new("cargo")
        .arg("build")
        .arg("-r")
        .current_dir(contest_dir)
        .status()?;
    Ok(status)
}
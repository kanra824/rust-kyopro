use std::env;
use std::process::{Command, ExitStatus};
use anyhow::Result;


pub fn build() -> Result<ExitStatus> {
    let contest_dir = env::var("CONTEST_DIR")?;
    let status = Command::new("cargo")
        .arg("build")
        .arg("-r")
        .current_dir(contest_dir)
        .status()?;
    Ok(status)
}
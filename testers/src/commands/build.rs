use std::env;
use std::process::{Command, ExitStatus};
use anyhow::Result;


pub fn build() -> Result<ExitStatus> {
    let current_dir = env::current_dir()?;
    let contest_dir = env::var("CONTEST_DIR")?;
    let dir = current_dir.join(contest_dir);
    let status = Command::new("cargo")
        .arg("build")
        .arg("-r")
        .current_dir(dir)
        .status()?;
    Ok(status)
}
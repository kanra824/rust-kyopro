use std::process::{Command, ExitStatus};
use anyhow::Result;


pub fn build(dir: String) -> Result<ExitStatus> {
    let status = Command::new("cargo")
        .arg("build")
        .current_dir(dir)
        .status()?;
    Ok(status)
}
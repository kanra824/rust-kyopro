use std::env;
use std::process::{Command, ExitStatus};
use anyhow::Result;

pub fn gen() -> Result<ExitStatus> {
    let current_dir = env::current_dir()?;
    let contest_dir = env::var("CONTEST_DIR")?;
    let dir = current_dir.join(contest_dir);
    let status = Command::new("cargo")
        .arg("run")
        .arg("-r")
        .arg("--bin")
        .arg("gen")
        .arg("seeds.txt")
        .current_dir(dir)
        .status()?;
    Ok(status)
}
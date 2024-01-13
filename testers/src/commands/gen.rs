use anyhow::Result;
use std::env;
use std::process::{Command, ExitStatus};

pub fn gen() -> Result<ExitStatus> {
    let contest_dir = env::var("CONTEST_DIR")?;
    let status = Command::new("cargo")
        .args(["run", "-r", "--bin", "gen", "seeds.txt"])
        .current_dir(contest_dir)
        .status()?;
    Ok(status)
}

use std::env;
use std::process::{Command, ExitStatus};
use anyhow::Result;

/// cargo run -r --bin tester ./a.out < in.txt > out.txt
pub fn _tester() -> Result<ExitStatus> {
    let current_dir = env::current_dir()?;
    let contest_dir = env::var("CONTEST_DIR")?;
    let dir = current_dir.join(contest_dir);
    let command_name = "a.cpp";
    let _ = Command::new("cargo")
        .args(&["run", "-r", "--bin", "tester", command_name, "<", "in/0000.txt", ">", "out/0000.txt"])
        .current_dir(dir)
        .status()?;
    unimplemented!();
}
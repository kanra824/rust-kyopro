
use anyhow::Result;
use std::env;
use std::process::{Command, ExitStatus};

/// cargo run -r --bin vis in.txt out.txt
pub fn vis(num: usize) -> Result<ExitStatus> {
    let contest_dir = env::var("CONTEST_DIR")?;
    let num = format!("{:0>4}", num);

    let input = format!("in/{}.txt", num);
    let output = format!("out/{}.txt", num);

    let status = Command::new("cargo")
        .args(["run", "-r", "--bin", "vis", &input, &output])
        .current_dir(contest_dir)
        .status()?;
    Ok(status)
}
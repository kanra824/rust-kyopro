use anyhow::Result;
use std::process::{Command, ExitStatus};
use std::fs::File;
use std::io::{Write, BufWriter};

pub fn gen_seed(contest_dir: String, n: usize) -> Result<ExitStatus> {
    let mut file = File::create(contest_dir + "/seeds.txt")?;
    for i in 0..n {
        writeln!(file, "{}", i)?;
    }
    Ok(ExitStatus::default())
}

pub fn gen(contest_dir: String) -> Result<ExitStatus> {
    let status = Command::new("cargo")
        .args(["run", "-r", "--bin", "gen", "seeds.txt"])
        .current_dir(contest_dir)
        .status()?;
    Ok(status)
}

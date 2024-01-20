use crate::commands::{exec::{exec, exec_all}, gen::gen, score::score};
use anyhow::Result;
use std::process::ExitStatus;
use std::sync::Arc;
use std::thread;

use super::score::score_all;

pub fn run(num: usize, contest_dir: &str, solver_path: &str) -> Result<ExitStatus> {
    gen(contest_dir.to_string().clone())?;
    exec(num, &contest_dir, &solver_path)?;
    let res = score(num, contest_dir.to_string().clone())?;
    eprintln!("{}", res);
    Ok(ExitStatus::default())
}

pub fn run_all(j: usize, contest_dir: String, solver_path: String) -> Result<ExitStatus> {
    gen(contest_dir.to_string().clone())?;
    let res = exec_all(j, contest_dir.clone(), solver_path)?;
    score_all(contest_dir)?;
    Ok(res)
}

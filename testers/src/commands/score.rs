use std::process::ExitStatus;
use anyhow::Result;
use std::io::Read;

pub fn score(num: usize, contest_dir: String) -> Result<i64> {
    let mut file = std::fs::File::open(format!("{}/score/{:0>4}.txt", contest_dir, num))?;
    let mut str = String::new();
    file.read_to_string(&mut str)?;
    let score = str.trim().parse::<i64>()?;
    Ok(score)
}

pub fn eprint_score(num: usize, contest_dir: String) -> Result<ExitStatus> {
    let score = score(num, contest_dir)?;
    eprintln!("{}", score);
    Ok(ExitStatus::default())
}

pub fn score_all (contest_dir: String) -> Result<ExitStatus> {
    let n = std::fs::read_dir(contest_dir.to_string() + "/score")?.count();

    let mut score_all = 0;
    for i in 0..n {
        let score = score(i, contest_dir.clone())?;
        eprintln!("{}: {}", i, score);
        score_all += score;
    }
    eprintln!("{}", score_all);

    Ok(ExitStatus::default())
}
use anyhow::Result;
use std::fs::File;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::Arc;
use std::thread;

/// [command] < in.txt > out.txt
pub fn exec(num: usize, contest_dir: &str, solver_path: &str) -> Result<ExitStatus> {
    let num = format!("{:0>4}", num);

    let input = File::open(format!("{}/in/{}.txt", contest_dir, num)).unwrap();
    let input = Stdio::from(input);

    std::fs::create_dir_all(contest_dir.to_string() + "/out")?;
    let output = File::create(format!("{}/out/{}.txt", contest_dir, num)).unwrap();
    let output = Stdio::from(output);

    let status = Command::new(solver_path)
        .stdin(input)
        .stdout(output)
        .current_dir(contest_dir)
        .status()?;
    Ok(status)
}

pub fn exec_all(j: usize, contest_dir: String, solver_path: String) -> Result<ExitStatus> {
    let n = std::fs::read_dir(contest_dir.to_string() + "/in")?.count();
    let mut res = Err(anyhow::anyhow!("n must be larger than 0"));
    let cd = Arc::new(contest_dir);
    let sp = Arc::new(solver_path);

    let mut num = 0;
    while num < n {
        let mut handler = vec![];
        for _ in 0..j {
            if num >= n {
                break;
            }
            println!("{}", num);
            let cd_i = cd.clone();
            let sp_i = sp.clone();
            handler.push(thread::spawn(move || {
                exec(num.clone(), cd_i.as_str(), sp_i.as_str())
            }));
            num += 1;
        }
        for h in handler {
            res = h.join().unwrap();
        }
    }
    res
}

use std::process::{Command, ExitStatus, Stdio};
use std::fs::File;
use std::os::unix::io::{AsRawFd, FromRawFd};
use anyhow::Result;
use std::thread;
use std::sync::Arc;

/// cargo run -r --bin tester ./a.out < in.txt > out.txt
pub fn tester(num: usize, contest_dir: &str, solver_path: &str) -> Result<ExitStatus> {
    let num = format!("{:0>4}", num);

    let input = File::open(format!("{}/in/{}.txt", contest_dir, num)).unwrap();
    let input = unsafe { Stdio::from_raw_fd(input.as_raw_fd()) };

    std::fs::create_dir_all(contest_dir.to_string() + "/out")?;
    let output = File::create(format!("{}/out/{}.txt", contest_dir, num)).unwrap();
    let output = unsafe { Stdio::from_raw_fd(output.as_raw_fd()) };

    let status = Command::new("cargo")
        .args(["run", "-r", "--bin", "tester", solver_path])
        .stdin(input)
        .stdout(output)
        .current_dir(contest_dir)
        .status()?;
    Ok(status)
}

pub fn tester_all(j: usize, contest_dir: String, solver_path: String) -> Result<ExitStatus> {
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
            let cd_i = cd.clone();
            let sp_i = sp.clone();
            handler.push(thread::spawn(move || {
                tester(num.clone(), cd_i.as_str(), sp_i.as_str())
            }));
            num += 1;
        }
        for h in handler {
            res = h.join().unwrap();
        }
    }
    res

}
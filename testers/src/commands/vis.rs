
use anyhow::Result;
use std::env;
use std::fs::File;
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};

/// cargo run -r --bin vis in.txt out.txt num
/// テスターが num を受け取るようにしておく
pub fn vis(num: usize) -> Result<ExitStatus> {
    let contest_dir = env::var("CONTEST_DIR")?;
    let num = format!("{:0>4}", num);

    let input = format!("in/{}.txt", num);
    let output = format!("out/{}.txt", num);

    std::fs::create_dir_all(contest_dir.to_string() + "/score")?;
    let score = File::create(format!("{}/score/{}.txt", contest_dir, num)).unwrap();
    let score = Stdio::from(score);

    // ビルドされてなければビルド
    if !Path::is_file(Path::new(&format!("{}/target/release/vis", &contest_dir))) {
        let status = Command::new("cargo")
            .args(["build", "-r"])
            .current_dir(contest_dir.clone())
            .status()?;

        if !status.success() {
            anyhow::bail!("'cargo build' failed in vis");
        }
    }

    // vis のバイナリを実行
    let status = Command::new(&format!("{}/target/release/vis", &contest_dir))
        .args([&input, &output, &num])
        .current_dir(contest_dir)
        .stderr(Stdio::null())
        .stdout(score)
        .status()?;

    Ok(status)
}

pub fn vis_all(contest_dir: String) -> Result<ExitStatus> {
    let n = std::fs::read_dir(contest_dir.to_string() + "/out")?.count();
    eprintln!("{}", n);
    let mut res = ExitStatus::default();
    for i in 0..n {
        res = vis(i)?;
    }
    Ok(res)
}
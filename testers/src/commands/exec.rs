use std::env;
use std::process::{Command, ExitStatus, Stdio};
use anyhow::Result;
use std::io::Write;

/// [command] [args]... < in.txt > out.txt
pub fn exec(num: usize, command: String, args: Option<Vec<String>>) -> Result<ExitStatus> {
    // set dir
    let current_dir = env::current_dir()?;
    let contest_dir = env::var("CONTEST_DIR")?;
    let contest_dir = current_dir.join(&contest_dir);
    let solver_dir = current_dir.parent().unwrap().to_str().unwrap();

    // set input / output path
    let in_path = contest_dir.join(format!("in/{:>04}.txt", num));
    let out_path = contest_dir.join(format!("out/{:>04}.txt", num));
    let _ = std::fs::create_dir(&contest_dir.join("out"));

    let args = args.unwrap_or_else(|| vec![]);
    let mut p = Command::new(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .current_dir(solver_dir)
        .spawn()?;
    let mut stdin = std::io::BufWriter::new(p.stdin.take().unwrap());
    let mut stdout = std::io::BufReader::new(p.stdout.take().unwrap());

    let input = std::fs::read_to_string(&in_path)?;
    writeln!(stdin, "{}", input)?;
    let _ = stdin.flush();

    // print p's stdout to output file
    let mut output_file = std::fs::File::create(&out_path)?;
    std::io::copy(&mut stdout, &mut output_file)?;

    let status = p.wait()?;
    Ok(status)
}
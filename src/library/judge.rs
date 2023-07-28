use crate::library::aoj::get_all_testcase_and_savefile;
use pipe;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::Fn;
use std::thread;
use std::time::{Duration, Instant};

enum Compare {
    EXACT,
}

#[derive(PartialEq, Eq)]
enum TestcaseResult {
    AC,
    WA,
    TLE,
    RE,
    MLE,
    CE,
    IE,
}

fn read_file(path: &String) -> anyhow::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn judge<F>(id: &str, solver: F) -> anyhow::Result<bool>
where
    F: Fn(),
{
    // testcase をダウンロード
    get_all_testcase_and_savefile(id, true);

    // testcase の個数を取得
    let path = format!(".cache_rust_kyopro/testcase_num/{id}.txt");
    let testcase_num = read_file(&path)?;
    let testcase_num = testcase_num.trim().parse::<i32>()?;

    // time limit を取得
    let path = format!(".cache_rust_kyopro/time_limit/{id}.txt");
    let time_limit = read_file(&path)?;
    let time_limit = time_limit.trim().parse::<i32>()?;

    let mut all_ac = true;
    for i in 0..testcase_num {
        let result = judge_testcase(id, i, time_limit, solver).unwrap();
        if result != TestcaseResult::AC {
            all_ac = false;
        }
    }

    Ok(all_ac)
}

fn judge_testcase<F>(
    id: &str,
    serial: i32,
    time_limit: i32,
    solver: F,
) -> anyhow::Result<TestcaseResult>
where
    F: Fn(),
{
    // input 読み込み
    let file_name_input = format!("save_input/{}/{}.txt", id, serial);
    let mut f_input = File::open(file_name_input)?;
    let mut input = String::new();
    f_input.read_to_string(&mut input)?;

    // output 読み込み
    let file_name_output = format!("save_output/{}/{}.txt", id, serial);
    let mut f_output = File::open(file_name_output)?;
    let mut output = String::new();
    f_output.read_to_string(&mut output)?;

    // 子プロセスを起動して solver を実行
    let (mut read, mut write) = pipe::pipe();
    let th = thread::spawn(move || {
        write.write_all(solver().as_bytes()).unwrap();
    });

    let start = Instant::now();
    th.join().unwrap();
    let end = start.elapsed();

    let mut result = String::new();
    read.read_to_string(&mut result).unwrap();

    // output と result を比較 compare(output, result);
    let res = compare(output, result, Compare::EXACT);
    Ok(TestcaseResult::AC)
}

fn compare(output: String, result: String, cmp: Compare) -> bool {
    output == result
}

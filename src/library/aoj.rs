use reqwest;
use serde::{de, Deserialize, Deserializer};
use std::any;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

fn remove_escape_of_newline<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.replace("\\n", "\n"))
}

#[derive(Deserialize, Debug)]
pub struct ProblemDescription {
    problem_id: String,
    time_limit: i32,
    memory_limit: i32,
}

#[derive(Deserialize, Debug)]
pub struct HeaderElm {
    serial: i32,
    name: String,
    #[serde(rename = "inputSize")]
    input_size: i32,
    #[serde(rename = "outputSize")]
    output_size: i32,
    score: i32,
}

#[derive(Deserialize, Debug)]
pub struct TestcaseHeader {
    #[serde(rename = "problemId")]
    problem_id: String,
    headers: Vec<HeaderElm>,
}

#[derive(Deserialize, Debug)]
pub struct Testcase {
    #[serde(rename = "problemId")]
    problem_id: String,
    serial: i32,
    #[serde(rename = "in", deserialize_with = "remove_escape_of_newline")]
    input: String,
    #[serde(rename = "out", deserialize_with = "remove_escape_of_newline")]
    output: String,
}

fn save_to_file(path: &Path, contents: &str) -> anyhow::Result<()> {
    // 必要なディレクトリを作成
    let prefix_input = path.parent().unwrap();
    std::fs::create_dir_all(prefix_input)?;

    // ファイルに保存
    let mut file = File::create(path)?;
    write!(file, "{}", contents)?;
    file.flush()?;
    Ok(())
}

// 問題の詳細を取得
async fn get_problem_description(id: &str) -> anyhow::Result<ProblemDescription> {
    let path = format!("https://judgeapi.u-aizu.ac.jp/resources/descriptions/Rust/{}", id);
    let body = reqwest::get(path).await?.json::<ProblemDescription>().await?;
    Ok(body)
}

// testcase の Header を取得する。これで問題数がわかる
async fn get_testcase_header(id: &str) -> anyhow::Result<TestcaseHeader> {
    let path = format!("https://judgedat.u-aizu.ac.jp/testcases/{}/header", id);
    let body = reqwest::get(path).await?.json::<TestcaseHeader>().await?;
    Ok(body)
}

async fn get_testcase(id: &str, serial: i32) -> anyhow::Result<Testcase> {
    let path = format!("https://judgedat.u-aizu.ac.jp/testcases/{}/{}", id, serial);
    let body = reqwest::get(path).await?.json::<Testcase>().await?;
    Ok(body)
}

async fn get_testcase_and_savefile(id: &str, serial: i32) -> anyhow::Result<()> {
    let body = get_testcase(id, serial).await?;

    let formatted_serial = format!("{:>04}", serial);

    let path_input = format!(".cache_rust_kyopro/input/{}/{}.txt", id, formatted_serial);
    let path_input = Path::new(&path_input);
    save_to_file(path_input, &body.input)?;

    let path_output = format!(".cache_rust_kyopro/output/{}/{}.txt", id, formatted_serial);
    let path_output = Path::new(&path_output);
    save_to_file(path_output, &body.output)?;

    Ok(())
}

pub async fn get_all_testcase_and_savefile(id: &str, use_cache: bool) -> anyhow::Result<()> {
    let testcase_header = get_testcase_header(id).await?;

    // テストケース数を保存
    let path = format!("save_testcase_num/{id}.txt");
    let path = Path::new(&path);
    // パスが存在すればダウンロード済みなのでreturn
    if path.exists() && use_cache {
        return Ok(());
    }
    save_to_file(path, &testcase_header.headers.len().to_string())?;

    // 各テストケースを取得して保存
    for header in testcase_header.headers {
        let serial = header.serial;
        eprintln!("downloading {:?}", header.name);
        get_testcase_and_savefile(id, serial).await?;
    }

    Ok(())
}

// ----- Test -----
#[tokio::test]
async fn test_get_problem_description() -> anyhow::Result<()> {
    let id = String::from("2439");
    let body = get_problem_description(&id).await?;
    eprintln!("{:?}", body);
    Ok(())
}

#[tokio::test]
async fn test_get_testcase_header() -> anyhow::Result<()> {
    let id = String::from("2439");
    let body = get_testcase_header(&id).await?;
    eprintln!("{:?}", body);
    Ok(())
}

#[tokio::test]
async fn test_get_testcase() -> anyhow::Result<()> {
    let id = String::from("2439");
    let serial = 23;
    let body = get_testcase(&id, serial).await?;
    eprintln!("{:?}", body);
    Ok(())
}

#[tokio::test]
async fn test_get_testcase_and_savefile() -> anyhow::Result<()> {
    let id = String::from("2439");
    let serial = 23;
    get_testcase_and_savefile(&id, serial).await?;
    Ok(())
}

#[tokio::test]
async fn test_get_all_testcases_and_savefile() -> anyhow::Result<()> {
    let id = String::from("2439");
    get_all_testcase_and_savefile(&id, true).await?;
    Ok(())
}

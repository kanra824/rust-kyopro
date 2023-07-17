use reqwest;
use serde::{de, Deserialize, Deserializer};
use serde_json::json;
use std::str::FromStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use anyhow::Result;


fn problem_id_from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    i32::from_str(&s).map_err(de::Error::custom)
}

fn remove_escape_of_newline<'de, D>(deserializer: D) -> Result<String, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    Ok(s.replace("\\n", "\n"))
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
    #[serde(rename = "problemId", deserialize_with = "problem_id_from_str")]
    problem_id: i32,
    headers: Vec<HeaderElm>
}

#[derive(Deserialize, Debug)]
pub struct Testcase {
    #[serde(rename = "problemId", deserialize_with = "problem_id_from_str")]
    problem_id: i32,
    serial: i32,
    #[serde(rename = "in", deserialize_with = "remove_escape_of_newline")]
    input: String,
    #[serde(rename = "out", deserialize_with = "remove_escape_of_newline")]
    output: String,
}

// testcase の Header を取得する。これで問題数がわかる
pub async fn get_testcase_header(id: i32) -> Result<TestcaseHeader> {
    let path = format!("https://judgedat.u-aizu.ac.jp/testcases/{}/header", id);
    let body = reqwest::get(path)
        .await?
        .json::<TestcaseHeader>()
        .await?;
    Ok(body)
}

pub async fn get_testcase(id: i32, serial: i32) -> Result<Testcase> {
    let path = format!("https://judgedat.u-aizu.ac.jp/testcases/{}/{}", id, serial);
    let body = reqwest::get(path)
        .await?
        .json::<Testcase>()
        .await?;
    Ok(body)
}

pub async fn get_testcase_and_savefile(id: i32, serial: i32) -> Result<()> {
    let body = get_testcase(id, serial)
                        .await?;

    eprintln!("{:?}", body);
    let formatted_serial = format!("{:>04}", serial);

    let path_input = format!("save_input/{}/{}.txt", id, formatted_serial);
    // 必要なディレクトリを作成
    let prefix_input = Path::new(&path_input).parent().unwrap();
    std::fs::create_dir_all(prefix_input)?;

    // ファイルに保存
    let mut file = File::create(path_input)?;
    write!(file, "{}", body.input)?;
    file.flush()?;

    let path_output = format!("save_output/{}/{}.txt", id, formatted_serial);
    // 必要なディレクトリを作成
    let prefix_output = Path::new(&path_output).parent().unwrap();
    std::fs::create_dir_all(prefix_output)?;

    // ファイルに保存
    let mut file = File::create(path_output)?;
    write!(file, "{}", body.output)?;
    file.flush()?;


    Ok(())
}

pub async fn get_all_testcase_and_savefile(id: i32) -> Result<()> {
    let testcase_header = get_testcase_header(id).await?;

    for header in testcase_header.headers {
        let serial = header.serial;
        eprintln!("downloading {:?}", header.name);
        get_testcase_and_savefile(id, serial).await?;
    }

    Ok(())
}


// ----- Test -----
#[tokio::test]
async fn test_get_testcase_header() -> Result<()> {
    let id = 2439;
    let body = get_testcase_header(id).await?;
    eprintln!("{:?}", body);
    Ok(())
}

#[tokio::test]
async fn test_get_testcase() -> Result<()> {
    let id = 2439;
    let serial = 23;
    let body = get_testcase(id, serial).await?;
    eprintln!("{:?}", body);
    Ok(())
}

#[tokio::test]
async fn test_get_testcase_and_savefile() -> Result<()> {
    let id = 2439;
    let serial = 23;
    get_testcase_and_savefile(id, serial).await?;
    Ok(())
}

#[tokio::test]
async fn test_get_all_testcases_and_savefile() -> Result<()> {
    let id = 2439;
    get_all_testcase_and_savefile(id).await?;
    Ok(())
}
use reqwest;

pub async fn get_testcase_header(id: usize) -> Result<String, reqwest::Error> {
    let path = format!("https://judgedat.u-aizu.ac.jp/testcases/{}/header", id);
    let body = reqwest::get(path)
        .await?
        .text()
        .await?;
    Ok(body)
}

// ----- Test -----
#[tokio::test]
async fn test_get_testcase_header() {
    let id = 2439;
    let body = get_testcase_header(id).await.unwrap();
    eprintln!("{}", body);
}
use reqwest::header::HeaderMap;
use reqwest::{self, Client, Url};
use serde::{Deserialize, Serialize};
use bat::PrettyPrinter;
// use serde_json::json;

pub async fn signin(id: &str) {
    let url = format!("https://at.kexie.space/api/user/signIn/")
        .parse::<Url>()
        .unwrap();
    let res = post(id, url).await.unwrap();
    let res: Output = serde_json::from_str(&res).unwrap();
    res.print();
}

pub async fn signout(id: &str) {
    let url = format!("https://at.kexie.space/api/user/signOut/")
        .parse::<Url>()
        .unwrap();
    let res = post(id, url).await.unwrap();
    let res: Output = serde_json::from_str(&res).unwrap();
    res.print();
}

async fn post(id: &str, url: Url) -> Result<String, reqwest::Error> {
    let body = format!("{{\"userId\":\"{id}\"}}");

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let res = Client::new()
        .post(url)
        .headers(headers)
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    Ok(res)
}
#[derive(Debug, Deserialize, Serialize)]
struct Output {
    data: Data,
    code: i32,
    msg: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct Data {
    #[serde(alias = "userId")]
    id: u32,
    #[serde(alias = "userName")]
    name: String,
    #[serde(alias = "totalTime")]
    total_time: String,
    #[serde(alias = "accumulatedTime")]
    this_time: Option<String>,
    #[serde(alias = "week")]
    week: u8,
}

impl Output {
    fn print(&self) {
        let mut output = format!("{}\n", self.msg);
        if let Some(v) = &self.data.this_time {
            output.push_str(format!("本次时长：{}\n", v).as_str());
        }
        output.push_str(format!("总时长：{}\n", self.data.total_time).as_str());
        PrettyPrinter::new().input_from_bytes(output.as_bytes()).language("json").print().unwrap();
    }
}

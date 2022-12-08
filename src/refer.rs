
use reqwest::{Client, Response, Url};
use serde::{Deserialize, Serialize};

pub async fn refer(id: &str) -> Result<(), reqwest::Error> {
    let url = format!("https://at.kexie.space/api/record/online/{}", id)
        .parse::<Url>()
        .unwrap();
    let res = get(url).await.unwrap().text().await.unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });
    let res: Output = serde_json::from_str(&res).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    res.print();
    Ok(())
}

pub async fn get(url: Url) -> Result<Response, reqwest::Error> {
    let res = Client::new().get(url).send().await;
    res
}

#[derive(Debug, Serialize, Deserialize)]
struct Output {
    data: Data,
    code: i32,
    msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    #[serde(alias = "userId")]
    id: u32,
    #[serde(alias = "userName")]
    name: String,
    status: Option<String>,
    start: Option<String>,
}

impl Output{
    fn print(&self){
        if let Some(status) = self.data.status.as_ref() {
            println!("{} {}",self.data.name,status);
        }

        if let Some(start_time) = self.data.start.as_ref() {
            println!("{}已就绪,开始时间：{}",self.data.name,start_time);
        }
    }
}
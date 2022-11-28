use reqwest::header::HeaderMap;
use reqwest::{self, Client, Url};
// use serde_json::json;

pub async fn signin(id: &str) {
    let url = format!("https://at.kexie.space/api/user/signIn/")
        .parse::<Url>()
        .unwrap();
    // println!("{:?}",url);
    let res = post(id, url).await.unwrap();
    println!("{:?}", res);
}

pub async fn signout(id: &str) {
    let url = format!("https://at.kexie.space/api/user/signOut/")
        .parse::<Url>()
        .unwrap();
    // println!("{:?}",url);
    let res = post(id, url).await.unwrap();
    println!("{:?}", res);
}

async fn post(id: &str, url: Url) -> Result<String, reqwest::Error> {
    let body = format!("{{\"userId\":\"{id}\"}}");
    println!("{body}");

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

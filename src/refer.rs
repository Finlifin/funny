use reqwest::{Client, Response, Url};
// use serde::{Deserialize, Serialize};

pub async fn refer(id: &str) -> Result<(), reqwest::Error> {
    let url = format!("https://at.kexie.space/api/record/online/{}", id)
        .parse::<Url>()
        .unwrap();
    let res = get(url).await.unwrap().text().await.unwrap();
    // let res = reqwest::get(url).await.unwrap().json::<TheJson>().await.unwrap();
    println!("{:#?}", res);
    Ok(())
}


pub async fn get(url: Url) -> Result<Response, reqwest::Error> {
    let res = Client::new().get(url).send().await;
    res
}

use reqwest::Url;
use crate::refer::{get, self};

pub async fn get_top_five() {
    let url = format!("https://at.kexie.space/api/record/topFive")
        .parse::<Url>()
        .unwrap();
    let res = get(url).await.unwrap().text().await.unwrap();
    println!("{:#?}",res);
}

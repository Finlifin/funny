use crate::refer::get;
use bat::PrettyPrinter;
use reqwest::Url;
use serde::{Deserialize, Serialize};

pub async fn get_top_five() {
    let url = format!("https://at.kexie.space/api/record/topFive")
        .parse::<Url>()
        .unwrap();
    let res = get(url)
        .await
        .unwrap()
        .text()
        .await
        .expect("请检查你的网络。");
    let res: Top = serde_json::from_str(&res).unwrap();
    print_data(&res);
    // println!("{:#?}", res);
}
#[derive(Debug, Deserialize, Serialize)]
struct Top {
    data: Vec<Person>,
    code: i32,
    msg: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    #[serde(alias = "userId")]
    id: u32,
    #[serde(alias = "userName")]
    name: String,
    #[serde(alias = "userDept")]
    dept: String,
    #[serde(alias = "userLocation")]
    location: String,
    #[serde(alias = "totalTime")]
    total_time: String,
    week: i32,
}

fn print_data(data: &Top) {
    for each in data.data.iter() {
        let line = format!("|=>{}{}:{}\n", each.dept, each.name, each.total_time);
        let _ = PrettyPrinter::new()
            .input_from_bytes(line.as_bytes())
            .language("python")
            .print()
            .unwrap();
    }
}

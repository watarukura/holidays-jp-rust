use chrono::{Local};

#[async_std::main]
async fn main() {
    let thisyear: String = Local::today().format("%Y").to_string();
    let uri = format!("{}{}", "https://api.national-holidays.jp/", thisyear);
    let mut res = surf::get(uri).await.unwrap();
    println!("{}", res.body_string().await.unwrap());
}
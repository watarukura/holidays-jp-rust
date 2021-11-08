use chrono::{Local};
use serde_derive::{Deserialize, Serialize};

pub type Holidays = Vec<Holiday>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Holiday {
    pub date: String,
    pub name: String,
}

#[async_std::main]
async fn main() {
    let thisyear: String = Local::today().format("%Y").to_string();
    let uri = format!("{}{}", "https://api.national-holidays.jp/", thisyear);
    let holidays: Holidays = surf::get(uri).recv_json().await.unwrap();
    dbg!(holidays);
}
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
    let today: String = Local::today().format("%Y-%m-%d").to_string();
    let uri = format!("{}{}", "https://api.national-holidays.jp/", thisyear);
    let holidays: Holidays = surf::get(uri).recv_json().await.unwrap();
    let iter = holidays.iter();
    let mut is_holiday = false;
    for holiday in iter {
        if holiday.date == today {
            is_holiday = true;
        }
    }
    dbg!(is_holiday);
}
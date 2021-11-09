use chrono::{Local, Duration};
use serde_derive::{Deserialize, Serialize};
use std::process::exit;

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
    let tomorrow: String = (Local::today() + Duration::days(1)).format("%Y-%m-%d").to_string();
    let uri = format!("{}{}", "https://api.national-holidays.jp/", thisyear);
    let holidays: Holidays = surf::get(uri).recv_json().await.unwrap();
    let is_holiday = holidays.iter().find(|h| h.date == today || h.date == tomorrow).is_some();
    println!("{}", is_holiday);

    let weekday_today: String = Local::today().format("%a").to_string();
    let weekday_tomorrow: String = (Local::today() + Duration::days(1)).format("%a").to_string();
    let weekend_names = ["Fri".to_string(), "Sat".to_string(), "Sun".to_string()];
    let is_weekend: bool = weekend_names.contains(&weekday_today) || weekend_names.contains(&weekday_tomorrow);
    println!("{}", is_weekend);

    let is_holiday_or_weekend = is_holiday || is_weekend;

    exit(match is_holiday_or_weekend {
        true => 1,
        _ => 0
    });
}
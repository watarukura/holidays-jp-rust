use chrono::{Local, Duration, TimeZone};
use serde_derive::{Deserialize, Serialize};
use std::process::exit;
use clap::{App, Arg};

pub type Holidays = Vec<Holiday>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Holiday {
    pub date: String,
    pub name: String,
}

#[async_std::main]
async fn main() {
    let matches = App::new("holidays-jp")
        .version("1.0.0")
        .arg(
            Arg::from("[Y-m-d] '(option) specify date'")
        ).get_matches();
    let specified_date = match matches.value_of("Y-m-d") {
        Some(date) => Local::datetime_from_str(&Local, &(date.to_string() + "T00:00:00+0900"), "%Y-%m-%dT%H:%M:%S+0900").unwrap(),
        None => Local::now(),
    };

    let today: String = specified_date.format("%Y-%m-%d").to_string();
    let tomorrow: String = (specified_date + Duration::days(1)).format("%Y-%m-%d").to_string();

    let uri = format!("{}{}", "https://api.national-holidays.jp/", "all");
    let holidays: Holidays = surf::get(uri).recv_json().await.unwrap();

    let is_holiday = holidays.iter().find(|h| h.date == today || h.date == tomorrow).is_some();
    // println!("{}", is_holiday);

    let weekday_today: String = specified_date.format("%a").to_string();
    let weekend_names = ["Fri".to_string(), "Sat".to_string(), "Sun".to_string()];
    let is_weekend: bool = weekend_names.contains(&weekday_today);
    // println!("{}", is_weekend);

    let is_holiday_or_weekend = is_holiday || is_weekend;

    exit(match is_holiday_or_weekend {
        true => 1,
        _ => 0
    });
}
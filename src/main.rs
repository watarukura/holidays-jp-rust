use chrono::{Local, TimeZone, DateTime};
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

    if is_holiday(specified_date, get_holidays().await) {
        println!("{}", "holiday");
        exit(1)
    }

    if is_weekend(specified_date) {
        println!("{}", "weekend");
        exit(1)
    }
    exit(0)
}

fn is_holiday(specified_date: DateTime<Local>, holidays: Holidays) -> bool {
    let today: String = specified_date.format("%Y-%m-%d").to_string();
    holidays.iter().find(|h| h.date == today).is_some()
}

async fn get_holidays() -> Holidays {
    let uri = format!("{}{}", "https://api.national-holidays.jp/", "all");
    surf::get(uri).recv_json().await.unwrap()
}

fn is_weekend(specified_date: DateTime<Local>) -> bool {
    let weekday_today = specified_date.format("%a").to_string();
    let weekend_names: Vec<String> = "Fri,Sat,Sun".split(",").map(|v| v.to_string()).collect();
    weekend_names.contains(&weekday_today)
}

#[cfg(test)]
mod tests {
    use chrono::{Local, TimeZone};
    use crate::{get_holidays, is_holiday, is_weekend};

    #[async_std::test]
    async fn is_holiday_test() {
        let new_year_day = Local::datetime_from_str(&Local, "2022-01-01T00:00:00+0900", "%Y-%m-%dT%H:%M:%S+0900").unwrap();
        assert!(is_holiday(new_year_day, get_holidays().await));
        let november_1st = Local::datetime_from_str(&Local, "2021-11-01T00:00:00+0900", "%Y-%m-%dT%H:%M:%S+0900").unwrap();
        assert!(!is_holiday(november_1st, get_holidays().await));
    }

    #[test]
    fn is_weekend_test() {
        let monday = Local::datetime_from_str(&Local, "2021-11-01T00:00:00+0900", "%Y-%m-%dT%H:%M:%S+0900").unwrap();
        let tuesday= Local::datetime_from_str(&Local, "2021-11-02T00:00:00+0900", "%Y-%m-%dT%H:%M:%S+0900").unwrap();
        let wednesday = Local::datetime_from_str(&Local, "2021-11-03T00:00:00+0900", "%Y-%m-%dT%H:%M:%S+0900").unwrap();
        let thursday = Local::datetime_from_str(&Local, "2021-11-04T00:00:00+0900", "%Y-%m-%dT%H:%M:%S+0900").unwrap();
        let friday = Local::datetime_from_str(&Local, "2021-11-05T00:00:00+0900", "%Y-%m-%dT%H:%M:%S+0900").unwrap();
        let saturday = Local::datetime_from_str(&Local, "2021-11-06T00:00:00+0900", "%Y-%m-%dT%H:%M:%S+0900").unwrap();
        let sunday = Local::datetime_from_str(&Local, "2021-11-07T00:00:00+0900", "%Y-%m-%dT%H:%M:%S+0900").unwrap();
        assert!(!is_weekend(monday));
        assert!(!is_weekend(tuesday));
        assert!(!is_weekend(wednesday));
        assert!(!is_weekend(thursday));
        assert!(is_weekend(friday));
        assert!(is_weekend(saturday));
        assert!(is_weekend(sunday));
    }
}
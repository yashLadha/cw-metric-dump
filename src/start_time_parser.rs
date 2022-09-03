use aws_sdk_cloudwatch::types::DateTime;
use chrono::{Duration, Utc};

fn decrease_time_by_days(days: i64) -> DateTime {
    DateTime::from_millis((Utc::now() - Duration::days(days)).timestamp_millis())
}

fn decrease_time_by_weeks(weeks: i64) -> DateTime {
    DateTime::from_millis((Utc::now() - Duration::weeks(weeks)).timestamp_millis())
}

fn decrease_time_by_hours(hours: i64) -> DateTime {
    DateTime::from_millis((Utc::now() - Duration::hours(hours)).timestamp_millis())
}

pub fn start_time_parse(start_time: Option<&String>) -> Option<DateTime> {
    if let Some(start_time_str) = start_time {
        let groups: Vec<&str> = start_time_str.split(':').collect();
        let time: i64 = groups[1].parse().unwrap();
        return match groups[0] {
            "d" => Some(decrease_time_by_days(time)),
            "h" => Some(decrease_time_by_hours(time)),
            "w" => Some(decrease_time_by_weeks(time)),
            _ => panic!("Invalid format passed {}", start_time_str),
        };
    }
    None
}

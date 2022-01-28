use regex::Regex;
use lazy_static::lazy_static;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp(value: &String) -> u64 {
    lazy_static! {
        static ref TIMESTAMP: Regex = Regex::new("\"eventTransmissionTime\":\\s*(\\d+),").unwrap();
    }
    TIMESTAMP.captures(&value).unwrap().get(1).unwrap().as_str().parse::<u64>().unwrap()
}

pub fn with_received_time(value: &String) -> String {
    let received_timestamp =
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("value older than EPOCH")
            .as_millis();
    let suffix = format!(",\"eventReceivedTime\":{}}}", received_timestamp);
    let prefix = value.trim_end().trim_end_matches("}").to_owned();
    prefix + &suffix
}

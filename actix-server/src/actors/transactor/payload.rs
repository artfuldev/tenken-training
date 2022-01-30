use std::time::{SystemTime, UNIX_EPOCH};
use gjson::get;

pub fn get_timestamp(value: &String) -> u64 {
    get(value, "eventTransmissionTime").u64()
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

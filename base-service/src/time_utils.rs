use std::time::SystemTime;

pub fn get_timestamp_now() -> i32 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i32
}

pub fn get_day_of_week(timestamp: i32) -> i32 {
    let rs: i32 = (timestamp / 86400).rem_euclid(7);
    (rs + 4).rem_euclid(7)
}

pub fn get_timestamp_dow(timestamp: i32, day: i32) -> i32 {
    let dow = get_day_of_week(timestamp);
    (timestamp + 86400 * (day - dow + 1)) / 86400 * 86400 - 3600 * 7
}

pub fn get_timestamp_next_week(timestamp: i32) -> i32 {
    timestamp + 86400 * 7
}

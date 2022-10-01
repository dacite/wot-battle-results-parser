use time::Duration;

/// `[0, 9, 15, 0]` => `"0_9_15_0"`
pub fn version_as_string(version: [u16; 4]) -> String {
    version.map(|x| x.to_string()).join("_")
}

pub fn get_replay_time(start_time: f64, current_time: f64, duration: i64) -> String {
    let total_time = Duration::minutes(duration);

    let actual_time = total_time - Duration::seconds_f64(current_time - start_time);

    format!(
        "{}:{:02}",
        actual_time.whole_minutes(),
        actual_time.whole_seconds() % 60
    )
}

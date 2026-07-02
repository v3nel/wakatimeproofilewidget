use chrono::{Duration, Local};

pub fn get_last_7_days_dates() -> (String, String) {
    let today = Local::now().date_naive();
    let seven_days_before = today - Duration::days(7);

    (
        today.format("%Y-%m-%d").to_string(),
        seven_days_before.format("%Y-%m-%d").to_string(),
    )
}
pub mod time_util {
    use chrono::{Utc, Datelike, TimeZone};

    pub fn start_of_current_month() -> i64 {
        let now = Utc::now();
        let start_of_month = Utc.with_ymd_and_hms(now.year(), now.month(), 1, 0, 0, 0);
        start_of_month.earliest().unwrap().timestamp()
    }
}

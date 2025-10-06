use chrono::{DateTime, Duration, Utc};

pub fn now() -> DateTime<Utc> {
    Utc::now()
}

pub fn now_plus_days(days: i64) -> DateTime<Utc> {
    now() + Duration::days(days)
}

pub fn now_plus_hours(hours: i64) -> DateTime<Utc> {
    now() + Duration::hours(hours)
}

pub fn format_datetime(dt: &DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

pub fn is_expired(expires_at: &DateTime<Utc>) -> bool {
    &now() > expires_at
}

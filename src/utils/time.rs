// convert timestamp to time ago
// less than a minute => just now
// less than an hour => x minutes ago
// less than a day => x hours ago
// less than a week => x days ago
// less than a month => x weeks ago
// less than a year => x months ago
// more than a year => x years ago
pub fn time_ago(timestamp: i64) -> String {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let diff = now - timestamp;

    if diff < 60 {
        "just now".to_string()
    } else if diff < 3600 {
        format!("{} minutes ago", diff / 60)
    } else if diff < 86400 {
        format!("{} hours ago", diff / 3600)
    } else if diff < 604800 {
        format!("{} days ago", diff / 86400)
    } else if diff < 2592000 {
        format!("{} weeks ago", diff / 604800)
    } else if diff < 31536000 {
        format!("{} months ago", diff / 2592000)
    } else {
        format!("{} years ago", diff / 31536000)
    }
}

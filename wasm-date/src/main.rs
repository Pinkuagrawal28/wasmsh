use anyhow::Result;
use chrono::{DateTime, Utc};
use std::time::SystemTime;

fn main() -> Result<()> {
    let now = SystemTime::now();
    let datetime: DateTime<Utc> = now.into();
    // e.g. 2023-10-27 10:00:00 UTC
    println!("{}", datetime.format("%Y-%m-%d %H:%M:%S %Z"));
    Ok(())
}

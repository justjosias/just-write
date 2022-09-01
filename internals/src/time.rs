//! Helper structures and functions for working with chrono
use chrono::Datelike;

#[derive(Debug)]
pub struct Timestamp {
    pub day: u32,
    pub month: u32,
    pub year: i32,
    /// An ISO timestamp string. Example: `2022-05-11T15:15:56+00:00`
    pub timestamp: String,
}

impl Timestamp {
    pub fn now() -> Self {
        let dt = chrono::Local::now();

        Self {
            day: dt.day(),
            month: dt.month(),
            year: dt.year(),
            timestamp: format!("{:?}", chrono::offset::Local::now()),
        }
    }
}

#[test]
fn test_timestamp() {
    println!("{:?}", Timestamp::now());
}

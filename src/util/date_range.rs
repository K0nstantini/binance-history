use std::mem;

use chrono::{DateTime, Duration, Utc};

pub struct DateRange(pub DateTime<Utc>, pub DateTime<Utc>);

impl Iterator for DateRange {
    type Item = DateTime<Utc>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + Duration::days(1);
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}
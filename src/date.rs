// date.rs

use chrono::{Duration, NaiveDate};

/// Represents a date along with its formatted string representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateInfo {
    /// The full `NaiveDate` instance.
    pub date: NaiveDate,
    /// The formatted date string in the "DDMMYY" format.
    pub formatted: String,
}

/// An iterator over dates in a given range (inclusive).
pub struct DateIterator {
    current: NaiveDate,
    end: NaiveDate,
}

impl Iterator for DateIterator {
    type Item = DateInfo;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            None
        } else {
            let date_info = DateInfo {
                date: self.current,
                formatted: self.current.format("%y%m%d").to_string(),
            };
            self.current = self.current + Duration::days(1);
            Some(date_info)
        }
    }
}

/// Returns an iterator that yields dates between the given start and end dates (inclusive).
///
/// The input dates must be in the "DD-MM-YYYY" format.
pub fn dates_between(start: &str, end: &str) -> DateIterator {
    let start_date =
        NaiveDate::parse_from_str(start, "%d-%m-%Y").expect("Invalid start date format");
    let end_date = NaiveDate::parse_from_str(end, "%d-%m-%Y").expect("Invalid end date format");
    DateIterator {
        current: start_date,
        end: end_date,
    }
}

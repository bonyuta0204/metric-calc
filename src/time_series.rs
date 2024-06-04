use chrono::{Date, Datelike, NaiveDate, ParseError};

#[derive(Debug)]
pub struct TimeSeries {
    pub periods: Vec<Period>,
}

#[derive(Debug)]
pub struct Period {
    idx: usize,
    date: NaiveDate,
    year: i32,
    month: u32,
    day: u32,
}

impl TimeSeries {
    pub fn from_iter<'a, I: Iterator<Item = &'a str>>(value: I) -> Result<Self, ParseError> {
        let mut periods: Vec<Period> = Vec::new();
        for (idx, v) in value.enumerate() {
            let date = NaiveDate::parse_from_str(v, "%Y-%m-%d")?;
            periods.push(Period {
                idx,
                date,
                year: date.year(),
                month: date.month(),
                day: date.day(),
            });
        }
        Ok(TimeSeries { periods })
    }
}

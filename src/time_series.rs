use std::{borrow::BorrowMut, collections::HashMap};

use chrono::{Date, Datelike, NaiveDate, ParseError};

use crate::period_mapper::{PeriodMapKind, PeriodMapper};

#[derive(Debug)]
pub struct TimeSeries {
    pub periods: Vec<Period>,
}

#[derive(Debug)]
pub struct Period {
    pub idx: usize,
    pub date: NaiveDate,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub period_map: HashMap<PeriodMapKind, usize>,
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
                period_map: HashMap::new(),
            });
        }
        Ok(TimeSeries { periods })
    }

    pub fn add_period_map(&mut self, period_mapper: impl PeriodMapper) {
        let period_mapping = period_mapper.map_period(&self.periods);
        for (i, period) in self.periods.iter_mut().enumerate() {
            if let Some(map_index) = period_mapping[i] {
                period
                    .period_map
                    .insert(period_mapper.map_kind(), map_index);
            }
        }
    }
}

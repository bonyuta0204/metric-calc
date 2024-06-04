use std::error::Error;

use crate::time_series::{Period, TimeSeries};

#[derive(Debug)]
pub struct Metric<'a> {
    label: String,
    values: Vec<MetricValue<'a>>,
}

#[derive(Debug)]
pub struct MetricValue<'a> {
    period: &'a Period,
    value: Option<i32>,
}

impl<'a> Metric<'a> {
    pub fn from_raw_data<'b, I: Iterator<Item = &'b str>>(
        label: &str,
        values: I,
        time_series: &'a TimeSeries,
    ) -> Result<Metric<'a>, Box<dyn Error>> {
        let values: Vec<Option<i32>> = values.map(|v| v.parse::<i32>().ok()).collect();
        let metric_values: Vec<_> = values
            .into_iter()
            .enumerate()
            .map(|(i, v)| MetricValue {
                period: &time_series.periods[i],
                value: v,
            })
            .collect();

        Ok(Metric {
            label: label.to_owned(),
            values: metric_values,
        })
    }
}

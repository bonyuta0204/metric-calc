use std::error::Error;

use crate::time_series::{Period, TimeSeries};

#[derive(Debug)]
pub struct Metric<'a> {
    pub label: String,
    pub values: Vec<MetricValue<'a>>,
}

#[derive(Debug)]
pub struct MetricValue<'a> {
    pub period: &'a Period,
    pub value: Option<f64>,
}

impl<'a> Metric<'a> {
    pub fn from_raw_data<'b, I: Iterator<Item = &'b str>>(
        label: &str,
        values: I,
        time_series: &'a TimeSeries,
    ) -> Result<Metric<'a>, Box<dyn Error>> {
        let values: Vec<Option<f64>> = values.map(|v| v.parse::<f64>().ok()).collect();
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

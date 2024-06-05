use std::{collections::HashMap, error::Error};

use crate::{
    metrics_calculator::{MetricCalculator, MetricValueKind},
    time_series::{Period, TimeSeries},
};

#[derive(Debug)]
pub struct Metric<'a> {
    pub label: String,
    pub values: Vec<MetricValue<'a>>,
}

#[derive(Debug)]
pub struct MetricValue<'a> {
    pub period: &'a Period,
    pub value: Option<f64>,
    pub calculated_values: HashMap<MetricValueKind, f64>,
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
                calculated_values: HashMap::new(),
            })
            .collect();

        Ok(Metric {
            label: label.to_owned(),
            values: metric_values,
        })
    }

    pub fn calculate_metric(&mut self, calculator: impl MetricCalculator) {
        let calc_result = calculator.calculate(self);
        for (i, metric_value) in self.values.iter_mut().enumerate() {
            let calc_value = calc_result[i];
            if let Some(v) = calc_value {
                metric_value
                    .calculated_values
                    .insert(calculator.metric_value_kind(), v);
            }
        }
    }
}

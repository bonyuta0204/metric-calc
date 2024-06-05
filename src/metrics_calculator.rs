use crate::{metrics::Metric, period_mapper::PeriodMapKind};

pub trait MetricCalculator {
    fn calculate(&self, metric: &Metric) -> Vec<Option<f64>>;
    fn metric_value_kind(&self) -> MetricValueKind;
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum MetricValueKind {
    PreviousMounthRatio,
}

pub struct PreviousMounthRatioCalculator {}

trait RatioMetricCalculator {
    fn period_map_kind(&self) -> PeriodMapKind;
    fn metric_value_kind(&self) -> MetricValueKind;
}

impl<T: RatioMetricCalculator> MetricCalculator for T {
    fn calculate(&self, metric: &Metric) -> Vec<Option<f64>> {
        metric
            .values
            .iter()
            .map(|v| -> Option<_> {
                let idx = v.period.period_map.get(&self.period_map_kind())?;
                let comparison_value = &metric.values[*idx];
                let target_value = v.value?;
                let comparison_value = comparison_value.value?;
                Some(target_value / comparison_value)
            })
            .collect()
    }

    fn metric_value_kind(&self) -> MetricValueKind {
        self.metric_value_kind()
    }
}

impl RatioMetricCalculator for PreviousMounthRatioCalculator {
    fn metric_value_kind(&self) -> MetricValueKind {
        MetricValueKind::PreviousMounthRatio
    }

    fn period_map_kind(&self) -> PeriodMapKind {
        PeriodMapKind::PerviousMonth
    }
}

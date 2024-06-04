use chrono::{Months, NaiveDate};

use crate::time_series::Period;

pub trait PeriodMapper {
    fn map_period(&self, periods: &Vec<Period>) -> Vec<Option<usize>>;
    fn map_kind(&self) -> PeriodMapKind;
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum PeriodMapKind {
    PerviousMonth,
}

pub struct PrevioustMonthMapper {}

// map the period with previous month. if same date does not exists, refer to the nearest date
impl PeriodMapper for PrevioustMonthMapper {
    fn map_period(&self, periods: &Vec<Period>) -> Vec<Option<usize>> {
        let period_map: Vec<_> = periods
            .iter()
            .map(|period| {
                let previous_date = period.date.checked_sub_months(Months::new(1))?;
                let index = periods
                    .iter()
                    .position(|period| period.date == previous_date);
                index
            })
            .collect();
        period_map
    }

    fn map_kind(&self) -> PeriodMapKind {
        PeriodMapKind::PerviousMonth
    }
}

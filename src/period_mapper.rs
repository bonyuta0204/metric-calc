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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time_series::{Period, TimeSeries};
    use chrono::NaiveDate;

    #[test]
    fn test_map_period() {
        let periods = vec![
            Period {
                idx: 0,
                date: NaiveDate::from_ymd_opt(2023, 2, 1).unwrap(),
                year: 2023,
                month: 3,
                day: 1,
                period_map: Default::default(),
            },
            Period {
                idx: 1,
                date: NaiveDate::from_ymd_opt(2023, 3, 1).unwrap(),
                year: 2023,
                month: 2,
                day: 1,
                period_map: Default::default(),
            },
        ];
        let mapper = PrevioustMonthMapper {};
        let result = mapper.map_period(&periods);
        assert_eq!(result, vec![None, Some(0)]);
    }

    #[test]
    fn test_map_kind() {
        let mapper = PrevioustMonthMapper {};
        assert_eq!(mapper.map_kind(), PeriodMapKind::PerviousMonth);
    }

    #[test]
    fn test_map_period_no_previous_month() {
        let periods = vec![
            Period {
                idx: 0,
                date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                year: 2023,
                month: 1,
                day: 1,
                period_map: Default::default(),
            },
            Period {
                idx: 1,
                date: NaiveDate::from_ymd_opt(2023, 3, 1).unwrap(),
                year: 2023,
                month: 3,
                day: 1,
                period_map: Default::default(),
            },
        ];
        let mapper = PrevioustMonthMapper {};
        let result = mapper.map_period(&periods);
        assert_eq!(result, vec![None, None]);
    }

    #[test]
    fn test_map_period_with_multiple_periods() {
        let periods = vec![
            Period {
                idx: 0,
                date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                year: 2023,
                month: 1,
                day: 1,
                period_map: Default::default(),
            },
            Period {
                idx: 1,
                date: NaiveDate::from_ymd_opt(2023, 2, 1).unwrap(),
                year: 2023,
                month: 2,
                day: 1,
                period_map: Default::default(),
            },
            Period {
                idx: 2,
                date: NaiveDate::from_ymd_opt(2023, 2, 2).unwrap(),
                year: 2023,
                month: 3,
                day: 1,
                period_map: Default::default(),
            },
        ];
        let mapper = PrevioustMonthMapper {};
        let result = mapper.map_period(&periods);
        assert_eq!(result, vec![None, Some(0), Some(0)]);
    }
}

use std::{error::Error, fs::File};

use chrono::{format::parse, ParseError};
use csv::StringRecord;
use metrics::Metric;
use metrics_calculator::PreviousMounthRatioCalculator;
use period_mapper::PrevioustMonthMapper;
use time_series::TimeSeries;
use timer::Timer;
mod metrics;
mod metrics_calculator;
mod period_mapper;
mod time_series;
mod timer;

fn main() -> Result<(), Box<dyn Error>> {
    let timer = Timer::new();
    let file = File::open("metrics.csv")?;
    let rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut records = rdr.into_records();

    let header = records.next().unwrap().expect("Failed to get header");
    println!("[{} ms] Start to parse header", timer.elapsed_ms_str());
    let mut time_series = parse_header_line(header)?;

    time_series.add_period_map(PrevioustMonthMapper {});
    println!("[{} ms] Finished to parse header", timer.elapsed_ms_str());

    let mut metrics: Vec<_> = records
        .map(|record| {
            let line = record.expect("Failed to get record");
            parse_body_line(line, &time_series).expect("Failed to parse body")
        })
        .collect();
    println!("[{} ms] Finished to parse metrics", timer.elapsed_ms_str());

    for metric in metrics.iter_mut() {
        metric.calculate_metric(PreviousMounthRatioCalculator {})
    }

    println!(
        "[{} ms] Finished to calculate metrics",
        timer.elapsed_ms_str()
    );

    dbg!(&metrics);
    Ok(())
}

fn parse_header_line(line: StringRecord) -> Result<TimeSeries, ParseError> {
    let mut header_iter = line.iter();

    // first cell is always '#'. So we skip it.
    header_iter.next();

    let series = TimeSeries::from_iter(header_iter)?;
    Ok(series)
}

fn parse_body_line(line: StringRecord, time_series: &TimeSeries) -> Result<Metric, Box<dyn Error>> {
    let mut body_iter = line.into_iter();
    // first cell is always '#'. So we skip it.
    let title = body_iter.next().unwrap_or_default();

    let metrics = Metric::from_raw_data(title, body_iter, time_series)?;
    Ok(metrics)
}

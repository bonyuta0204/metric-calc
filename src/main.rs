use std::{error::Error, fs::File};

use chrono::{format::parse, ParseError};
use csv::StringRecord;
use metrics::Metric;
use time_series::TimeSeries;
mod metrics;
mod time_series;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("metrics.csv")?;
    let rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut records = rdr.into_records();

    let header = records.next().unwrap().expect("Failed to get header");
    let time_series = parse_header_line(header)?;

    let metrics: Vec<_> = records
        .map(|record| {
            let line = record.expect("Failed to get record");
            parse_body_line(line, &time_series).expect("Failed to parse body")
        })
        .collect();

    dbg!(metrics);

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

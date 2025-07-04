use chrono::{DateTime, Utc};
use fitparser;
use fitparser::{FitDataRecord, Value};
use pyo3::prelude::*;
use std::fs::File;
use std::path::Path;

const SEMICIRCLES_TO_DEGREES: f64 = 180.0 / (1u64 << 31) as f64;

// Extension trait for rounding f64 to n decimal places
trait RoundTo {
    fn round_to(self, decimals: i32) -> Self;
}

impl RoundTo for f64 {
    fn round_to(self, decimals: i32) -> Self {
        let factor = 10f64.powi(decimals);
        (self * factor).round() / factor
    }
}

fn get_data(path: &str) -> PyResult<Vec<FitDataRecord>> {
    let path = Path::new(path);
    let mut fit_file = File::open(path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

    let records = fitparser::from_reader(&mut fit_file)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    Ok(records)
}


fn get_coordinates(records: &[FitDataRecord]) -> Vec<(f64, f64)> {
    let mut coordinates: Vec<(f64, f64)> = Vec::new();
    for record in records.iter() {
        let mut lat: Option<i32> = None;
        let mut lon: Option<i32> = None;

        for field in record.fields() {
            match field.name() {
                "position_lat" => {
                    if let Value::SInt32(v) = field.value() {
                        lat = Some(*v);
                    }
                }
                "position_long" => {
                    if let Value::SInt32(v) = field.value() {
                        lon = Some(*v)
                    }
                }
                _ => continue,
            }
            if lat.is_some() && lon.is_some() {
                break;
            }
        }

        if let (Some(lat_val), Some(lon_val)) = (lat, lon) {
            let lat_deg = (lat_val as f64 * SEMICIRCLES_TO_DEGREES).round_to(5);
            let lon_deg = (lon_val as f64 * SEMICIRCLES_TO_DEGREES).round_to(5);
            coordinates.push((lon_deg, lat_deg));
        }
    }
    coordinates
}


fn get_timestamp(records: &[FitDataRecord]) -> Option<String> {
    for record in records {
        for field in record.fields() {
            if field.name() == "timestamp" {
                if let Value::Timestamp(ts) = field.value() {
                    let dt: DateTime<Utc> = (*ts).into();
                    return Some(dt.to_rfc3339());
                }
            }
        }
    }
    None
}

// Parses fit file and return [(longtiture, latitude), ...]
#[pyfunction]
fn parse_coordinates(path: &str) -> PyResult<Vec<(f64, f64)>> {
    let records = get_data(path)?;
    Ok(get_coordinates(&records))
}

/// Extract timestamp from .FIT file as a string in ISO 8601 format.
#[pyfunction]
fn parse_timestamp(path: &str) -> PyResult<Option<String>> {
    let records = get_data(path)?;
    Ok(get_timestamp(&records))
}


/// A Python module implemented in Rust.
#[pymodule]
fn parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_coordinates, m)?)?;
    m.add_function(wrap_pyfunction!(parse_timestamp, m)?)?;
    Ok(())
}

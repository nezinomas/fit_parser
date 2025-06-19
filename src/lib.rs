use pyo3::prelude::*;
use std::fs::File;
use fitparser;
use fitparser::{Value};
use std::path::Path;
use chrono::{DateTime, Utc};

const SEMICIRCLES_TO_DEGREES: f64 = 180.0 / (1u64 << 31) as f64;


#[pyfunction]
fn parse_coordinates(path: &str) -> PyResult<Vec<(f64, f64)>> {
    // let mut fit_file = File::open("a.fit")?;
    let path = Path::new(path);
    let mut fit_file = File::open(path).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

    let mut coordinates: Vec<(f64, f64)> = Vec::new();
    let records = fitparser::from_reader(&mut fit_file).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    for record in records {

        let fields = record.fields();
        let mut lat: Option<i32> = None;
        let mut lon: Option<i32> = None;

        for field in fields {
            match field.name() {
                "position_lat" => if let Value::SInt32(v) = field.value() {
                    lat = Some(*v);
                }
                "position_long" => if let Value::SInt32(v) = field.value() {
                    lon = Some(*v)
                },
                _ => continue,
            }
        }

        if let (Some(lat_val), Some(lon_val)) = (lat, lon) {
            let lat_deg = (lat_val as f64 * SEMICIRCLES_TO_DEGREES).round_to(5);
              let lon_deg = (lon_val as f64 * SEMICIRCLES_TO_DEGREES).round_to(5);
            coordinates.push((lon_deg, lat_deg));
        }
    }
    Ok(coordinates)
}


/// Extract timestamp from .FIT file as a string in ISO 8601 format.
#[pyfunction]
fn parse_timestamp(path: &str) -> PyResult<Option<String>> {
    let path = Path::new(path);
    let mut file = File::open(path).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

    let records = fitparser::from_reader(&mut file).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    for record in records {
        let fields = record.fields();
        for field in fields {
            if field.name() == "timestamp" {
                if let Value::Timestamp(ts) = field.value() {
                    let dt: DateTime<Utc> = (*ts).into();
                    return Ok(Some(dt.to_rfc3339()));
                }
            }
        }
    }
    Ok(None)
}

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

/// A Python module implemented in Rust.
#[pymodule]
fn parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_coordinates, m)?)?;
    m.add_function(wrap_pyfunction!(parse_timestamp, m)?)?;
    Ok(())
}

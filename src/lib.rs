extern crate geohash;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use geohash::{decode, encode, neighbor, Coordinate, Direction};

/// Encode coords into geohash
#[pyfunction]
fn geohash_encode(x: f64, y: f64, level: usize) -> PyResult<String> {
    let c = Coordinate { x: x, y: y };
    let geohash_string = geohash::encode(c, level).expect("Invalid coordinate");
    Ok(geohash_string)
}

// Decode geohash into Python list
#[pyfunction]
fn geohash_decode(hash: &str) -> PyResult<Vec<f64>> {
    let (c, _, _) = decode(&hash).expect("Invalid hash string");
    let vec = vec![c.x, c.y];
    Ok(vec)
}

/// My first Python module implemented in Rust.
#[pymodule]
fn geohashrs(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(geohash_encode))?;
    m.add_wrapped(wrap_pyfunction!(geohash_decode))?;
    Ok(())
}

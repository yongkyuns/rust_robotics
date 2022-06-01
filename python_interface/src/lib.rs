#![allow(non_snake_case)]
use pyo3::{exceptions, prelude::*};

use numpy::{ndarray::Axis, PyArray1};
// use rust_robotics::Vector4;

#[pyfunction]
fn LQR_control(states: &PyArray1<f64>) -> PyResult<f64> {
    let states = states.to_owned_array();

    if states.len_of(Axis(0)) != 4 {
        Err(exceptions::PyTypeError::new_err("Input size must be 4"))
    } else {
        Ok(0.0)
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn array_add(input: &PyArray1<f64>) -> PyResult<f64> {
    let mut val = 0.0;
    for v in input.to_owned_array().iter() {
        val += *v;
    }
    Ok(val)
}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyrust_robot(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(array_add, m)?)?;
    m.add_function(wrap_pyfunction!(LQR_control, m)?)?;
    Ok(())
}

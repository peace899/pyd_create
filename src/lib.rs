use pyo3::{exceptions::PyValueError, prelude::*};

/// This function is callable from Python.
#[pyfunction]
fn run() -> PyResult<()> {
    println!("Hello from Rust! The function 'run()' was called.");
    Ok(())
}

#[pyfunction]
fn add(a: i32, b: i32) -> PyResult<i32> {
    Ok(a + b)
}

#[pyfunction]
fn divide(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        return Err(PyValueError::new_err("Cannot divide by zero"));
    }
    Ok(a / b)
}

#[pyclass]
struct Counter {
    value: u64,
}

#[pymethods]
impl Counter {
    #[new]
    fn new() -> Self {
        Counter { value: 0 }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn get_value(&self) -> u64 {
        self.value
    }
}

/// The module's entry point. The function's name must match the library's name.
#[pymodule]
fn my_extension(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(divide, m)?)?;
    m.add_class::<Counter>()?;
    Ok(())
}
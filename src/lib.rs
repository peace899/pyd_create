use pyo3::prelude::*;

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

/// The module's entry point. The function's name must match the library's name.
#[pymodule]
fn my_extension(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
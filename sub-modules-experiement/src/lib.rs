use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


fn register_child_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new_bound(parent_module.py(), "child_module")?;
    child_module.add_function(wrap_pyfunction!(sum_as_string, &child_module)?)?;
    parent_module.add_submodule(&child_module)?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn new_project(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_child_module(m)?;
    Ok(())
}

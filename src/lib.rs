use pyo3::prelude::*;
use crate::packages_2::plus_array as rust_plust_array;
pub mod packages_2;


/// This function takes two unsigned integer parameters, `a` and `b`, and returns their sum as a string.
///
/// # Parameters
///
/// * `a` - An unsigned integer representing the first number.
/// * `b` - An unsigned integer representing the second number.
///
/// # Return
///
/// * `Ok(String)` - A string representation of the sum of `a` and `b`.
///
/// # Errors
///
/// This function does not return any errors.
#[pyfunction]
pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// plus_array(list_1, b, /)
/// --
///
/// This function takes a list of integers and an integer `b` as input, and returns a new list where each element is the original element plus `b`.
///
/// # Parameters
///
/// * `list_1` - A vector of integers representing the input list.
/// * `b` - An integer representing the value to be added to each element in `list_1`.
///
/// # Return
///
/// * `Ok(Vec<i32>)` - A vector of integers representing the result of adding `b` to each element in `list_1`.
///
/// # Errors
///
/// This function does not return any errors.
#[pyfunction(text_signature = "(list_1: list[int], b: int, /)")]
pub fn plus_array(list_1: Vec<i32>, b: i32) -> PyResult<Vec<i32>> {
    Ok(rust_plust_array(list_1, b))
}


/// A Python module implemented in Rust. Also, we can add documentation here.
#[pymodule]
fn my_rust_python_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(plus_array, m)?)?;
    Ok(())
}

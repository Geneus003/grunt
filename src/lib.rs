use pyo3::prelude::*;

pub mod generators;
pub mod types;

#[pyfunction]
fn generate_3d_model() -> PyResult<i32> {
    println!("hello, i'm module");
    return Ok(10)
}

#[pymodule]
fn grunt(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_3d_model, m)?)?;
    m.add_class::<types::Axis>()?;
    Ok(())
}

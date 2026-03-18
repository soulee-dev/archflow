use pyo3::prelude::*;

#[pyfunction]
fn render_svg(json_ir: &str) -> PyResult<String> {
    archflow_core::render_svg(json_ir)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

#[pymodule]
fn _archflow_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(render_svg, m)?)?;
    Ok(())
}

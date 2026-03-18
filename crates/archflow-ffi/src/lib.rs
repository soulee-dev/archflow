use pyo3::prelude::*;

#[pyfunction]
fn render_svg(json_ir: &str) -> PyResult<String> {
    archflow_core::render_svg(json_ir)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

#[pyfunction]
fn render_dsl(dsl: &str) -> PyResult<String> {
    archflow_core::render_dsl(dsl)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

#[pyfunction]
fn parse_dsl(dsl: &str) -> PyResult<String> {
    archflow_core::parse_dsl_to_json(dsl)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

#[pymodule]
fn _archflow_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(render_svg, m)?)?;
    m.add_function(wrap_pyfunction!(render_dsl, m)?)?;
    m.add_function(wrap_pyfunction!(parse_dsl, m)?)?;
    Ok(())
}

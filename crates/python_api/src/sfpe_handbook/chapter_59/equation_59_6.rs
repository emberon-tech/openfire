use pyo3::prelude::*;

use openfire::sfpe_handbook::chapter_59::equation_59_6 as rust_equation_59_6;

#[pyfunction]
/// Calculates specific flow of evacuating persons (Equation 59.6).
///
/// Specific flow is the flow of evacuating persons past a point in the exit
/// route per unit of time per unit of effective width.
///
/// .. math::
///
///    F_s = S \cdot D
///
/// where:
///
/// - :math:`F_s` is the specific flow (persons/min/ft or persons/s/m of effective width)
/// - :math:`S` is the speed along the line of travel (from Equation 59.5)
/// - :math:`D` is the population density (persons per unit area)
///
/// Args:
///     s (float): Speed along the line of travel (m/s or ft/min)
///     d (float): Population density (persons/m² or persons/ft²)
///
/// Returns:
///     float: Specific flow Fs (persons/s/m or persons/min/ft of effective width)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_59.equation_59_6.specific_flow(1.0, 0.5)
///     >>> print(f"{result:.2f} persons/s/m")
fn specific_flow(s: f64, d: f64) -> PyResult<f64> {
    Ok(rust_equation_59_6::specific_flow(s, d))
}

#[pymodule]
pub fn equation_59_6(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(specific_flow, m)?)?;
    Ok(())
}

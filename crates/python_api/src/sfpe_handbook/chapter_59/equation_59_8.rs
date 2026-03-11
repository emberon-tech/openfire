use pyo3::prelude::*;

use openfire::sfpe_handbook::chapter_59::equation_59_8 as rust_equation_59_8;

#[pyfunction]
/// Calculates the flow rate of persons passing a point in an exit route (Equation 59.8).
///
/// The calculated flow is the predicted flow rate of persons passing a particular
/// point in an exit route. This equation assumes the achievable flow rate through
/// a component is directly proportional to its width.
///
/// .. math::
///
///    F_c = F_s \cdot W_e
///
/// where:
///
/// - :math:`F_c` is the calculated flow (persons/min or persons/s)
/// - :math:`F_s` is the specific flow (persons/min/ft or persons/s/m of effective width)
/// - :math:`W_e` is the effective width of the component being traversed (ft or m)
///
/// Args:
///     fs (float): Specific flow (persons/min/ft or persons/s/m of effective width)
///     we (float): Effective width of the component being traversed (ft or m)
///
/// Returns:
///     float: Calculated flow Fc (persons/min or persons/s)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_59.equation_59_8.calculated_flow(1.3, 1.5)
///     >>> print(f"{result:.2f} persons/s")
fn calculated_flow(fs: f64, we: f64) -> PyResult<f64> {
    Ok(rust_equation_59_8::calculated_flow(fs, we))
}

#[pymodule]
pub fn equation_59_8(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(calculated_flow, m)?)?;
    Ok(())
}

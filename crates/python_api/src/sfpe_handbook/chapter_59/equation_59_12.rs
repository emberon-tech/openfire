use pyo3::prelude::*;

use openfire::sfpe_handbook::chapter_59::equation_59_12 as rust_equation_59_12;

#[pyfunction]
/// Calculates the specific flow departing from a transition point (Equation 59.12).
///
/// For cases involving one flow into and one flow out of a transition point,
/// this equation determines the specific flow of the route departing from
/// the transition point.
///
/// .. math::
///
///    F_{s(out)} = \frac{F_{s(in)} \cdot W_{e(in)}}{W_{e(out)}}
///
/// where:
///
/// - :math:`F_{s(out)}` is the specific flow departing from transition point (persons/min/ft or persons/s/m)
/// - :math:`F_{s(in)}` is the specific flow arriving at transition point (persons/min/ft or persons/s/m)
/// - :math:`W_{e(in)}` is the effective width prior to transition point (ft or m)
/// - :math:`W_{e(out)}` is the effective width after passing transition point (ft or m)
///
/// Args:
///     fs_in (float): Specific flow arriving at transition point (persons/min/ft or persons/s/m)
///     we_in (float): Effective width prior to transition point (ft or m)
///     we_out (float): Effective width after passing transition point (ft or m)
///
/// Returns:
///     float: Specific flow departing from transition point Fs(out) (persons/min/ft or persons/s/m)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_59.equation_59_12.specific_flow_out(1.0, 2.0, 1.0)
///     >>> print(f"{result:.1f} persons/s/m")
fn specific_flow_out(fs_in: f64, we_in: f64, we_out: f64) -> PyResult<f64> {
    Ok(rust_equation_59_12::specific_flow_out(fs_in, we_in, we_out))
}

#[pymodule]
pub fn equation_59_12(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(specific_flow_out, m)?)?;
    Ok(())
}

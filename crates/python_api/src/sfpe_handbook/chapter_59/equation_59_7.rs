use pyo3::prelude::*;

use openfire::sfpe_handbook::chapter_59::equation_59_7 as rust_equation_59_7;

#[pyfunction]
/// Calculates specific flow by combining Equations 59.5 and 59.6 (Equation 59.7).
///
/// This equation combines the travel speed equation (59.5) with the specific
/// flow equation (59.6) to express specific flow directly in terms of density.
///
/// .. math::
///
///    F_s = (1 - aD) \cdot k \cdot D
///
/// where:
///
/// - :math:`F_s` is the specific flow (persons/min/ft or persons/s/m of effective width)
/// - :math:`a` is the unit conversion constant (0.266 for SI, 2.86 for imperial)
/// - :math:`D` is the population density (persons per unit area)
/// - :math:`k` is the speed constant from Table 59.2
///
/// Args:
///     a (float): Unit conversion constant (0.266 for SI units, 2.86 for imperial units)
///     d (float): Population density (persons/m² or persons/ft²)
///     k (float): Speed constant from Table 59.2
///
/// Returns:
///     float: Specific flow Fs (persons/s/m or persons/min/ft of effective width)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_59.equation_59_7.specific_flow(0.266, 0.5, 1.19)
///     >>> print(f"{result:.4f} persons/s/m")
fn specific_flow(a: f64, d: f64, k: f64) -> PyResult<f64> {
    Ok(rust_equation_59_7::specific_flow(a, d, k))
}

#[pymodule]
pub fn equation_59_7(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(specific_flow, m)?)?;
    Ok(())
}

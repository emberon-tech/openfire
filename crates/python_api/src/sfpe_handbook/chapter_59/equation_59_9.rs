use pyo3::prelude::*;

use openfire::sfpe_handbook::chapter_59::equation_59_9 as rust_equation_59_9;

#[pyfunction]
/// Calculates flow rate by combining Equations 59.7 and 59.8 (Equation 59.9).
///
/// This equation combines the specific flow equation (59.7) with the calculated
/// flow equation (59.8) to express flow rate directly in terms of the fundamental
/// parameters.
///
/// .. math::
///
///    F_c = (1 - aD) \cdot k \cdot D \cdot W_e
///
/// where:
///
/// - :math:`F_c` is the calculated flow (persons/min or persons/s)
/// - :math:`a` is constant (0.266 for SI, 2.86 for imperial)
/// - :math:`D` is the population density (persons per unit area)
/// - :math:`k` is the constant from Table 59.2
/// - :math:`W_e` is the effective width of the component (ft or m)
///
/// Fc is in persons/min when k = k1 (from Table 59.2), D is in persons/ft², and We is in ft.
/// Fc is in persons/s when k = k2 (from Table 59.2), D is in persons/m², and We is in m.
///
/// Args:
///     a (float): Unit constant (0.266 for SI units, 2.86 for imperial units)
///     d (float): Population density (persons/m² or persons/ft²)
///     k (float): Constant from Table 59.2
///     we (float): Effective width of the component being traversed (ft or m)
///
/// Returns:
///     float: Calculated flow Fc (persons/min or persons/s)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_59.equation_59_9.calculated_flow(0.266, 0.5, 1.40, 1.2)
///     >>> print(f"{result:.4f} persons/s")
fn calculated_flow(a: f64, d: f64, k: f64, we: f64) -> PyResult<f64> {
    Ok(rust_equation_59_9::calculated_flow(a, d, k, we))
}

#[pymodule]
pub fn equation_59_9(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(calculated_flow, m)?)?;
    Ok(())
}

use pyo3::prelude::*;

use openfire::sfpe_handbook::chapter_59::equation_59_5 as rust_equation_59_5;

#[pyfunction]
/// Calculates pedestrian travel speed along a path (Equation 59.5).
///
/// .. math::
///
///    S = k - akD
///
/// where:
///
/// - :math:`S` is the speed along the line of travel
/// - :math:`k` is the speed constant from Table 59.2
/// - :math:`a` is the unit conversion constant (0.266 for SI, 2.86 for imperial)
/// - :math:`D` is the population density in persons per unit area
///
/// Args:
///     k (float): Speed constant from Table 59.2
///     a (float): Unit conversion constant (0.266 for SI, 2.86 for imperial)
///     d (float): Population density (persons per unit area)
///
/// Returns:
///     float: Speed S along the line of travel
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_59.equation_59_5.travel_speed(1.19, 0.266, 0.5)
///     >>> print(f"{result:.3f} m/s")
fn travel_speed(k: f64, a: f64, d: f64) -> PyResult<f64> {
    Ok(rust_equation_59_5::travel_speed(k, a, d))
}

#[pymodule]
pub fn equation_59_5(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(travel_speed, m)?)?;
    Ok(())
}

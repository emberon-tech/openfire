use pyo3::prelude::*;

use openfire::sfpe_handbook::chapter_59::equation_59_11 as rust_equation_59_11;

#[pyfunction]
/// Calculates the time for passage combining Equations 59.9 and 59.10 (Equation 59.11).
///
/// This equation combines the calculated flow equation (59.9) with the time for
/// passage equation (59.10) to express time directly in terms of the fundamental
/// parameters.
///
/// .. math::
///
///    t_p = \frac{P}{(1 - aD) \cdot k \cdot D \cdot W_e}
///
/// where:
///
/// - :math:`t_p` is the time for passage (minutes or seconds)
/// - :math:`P` is the population size (persons)
/// - :math:`a` is constant (0.266 for SI, 2.86 for imperial)
/// - :math:`D` is the population density (persons per unit area)
/// - :math:`k` is the constant from Table 59.2
/// - :math:`W_e` is the effective width of the component (ft or m)
///
/// tp is in minutes when k = k1 (from Table 59.2), D is in persons/ft², and We is in ft.
/// tp is in seconds when k = k2 (from Table 59.2), D is in persons/m², and We is in m.
///
/// Args:
///     p (float): Population size (persons)
///     a (float): Unit constant (0.266 for SI units, 2.86 for imperial units)
///     d (float): Population density (persons/m² or persons/ft²)
///     k (float): Constant from Table 59.2
///     we (float): Effective width of the component being traversed (ft or m)
///
/// Returns:
///     float: Time for passage tp (minutes or seconds depending on units)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_59.equation_59_11.time_for_passage(100.0, 0.266, 0.5, 1.40, 1.2)
///     >>> print(f"{result:.1f} seconds")
fn time_for_passage(p: f64, a: f64, d: f64, k: f64, we: f64) -> PyResult<f64> {
    Ok(rust_equation_59_11::time_for_passage(p, a, d, k, we))
}

#[pymodule]
pub fn equation_59_11(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(time_for_passage, m)?)?;
    Ok(())
}

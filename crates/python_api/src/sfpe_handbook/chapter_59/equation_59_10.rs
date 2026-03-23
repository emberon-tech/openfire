use pyo3::prelude::*;

use openfire::sfpe_handbook::chapter_59::equation_59_10 as rust_equation_59_10;

#[pyfunction]
/// Calculates the time for passage (Equation 59.10).
///
/// The time for passage is the time for a group of persons to pass a point
/// in an exit route.
///
/// .. math::
///
///    t_p = \frac{P}{F_c}
///
/// where:
///
/// - :math:`t_p` is the time for passage (minutes or seconds)
/// - :math:`P` is the population size (persons)
/// - :math:`F_c` is the calculated flow (persons/min or persons/s)
///
/// The units of tp depend on the units of Fc: tp is in minutes when Fc is in
/// persons/min; tp is in seconds when Fc is in persons/s.
///
/// Args:
///     p (float): Population size (persons)
///     fc (float): Calculated flow (persons/min or persons/s)
///
/// Returns:
///     float: Time for passage tp (minutes or seconds depending on Fc units)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_59.equation_59_10.time_for_passage(100.0, 20.0)
///     >>> print(f"{result:.1f} minutes")
fn time_for_passage(p: f64, fc: f64) -> PyResult<f64> {
    Ok(rust_equation_59_10::time_for_passage(p, fc))
}

#[pymodule]
pub fn equation_59_10(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(time_for_passage, m)?)?;
    Ok(())
}

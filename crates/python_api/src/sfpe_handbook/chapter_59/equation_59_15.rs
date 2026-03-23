use pyo3::prelude::*;

use openfire::sfpe_handbook::chapter_59::equation_59_15 as rust_equation_59_15;

#[pyfunction]
/// Calculates evacuation time when congestion dominates (Equation 59.15).
///
/// In Scenario 1 it is assumed that congestion dominates the results produced.
/// In such situations the time required for the evacuation of an enclosure
/// depends on the pre-evacuation time and unrestricted walking time of the
/// first few occupants to start to leave; these determine the time for
/// congestion to develop.
///
/// .. math::
///
///    t_{esc} = t_{pe1} + t_{trav} + t_{flow}
///
/// where:
///
/// - :math:`t_{esc}` is the evacuation time for an enclosure (time units)
/// - :math:`t_{pe1}` is the pre-evacuation time associated with the first people to respond (time units)
/// - :math:`t_{trav}` is the time taken to traverse the average distance to a place of safety (time units)
/// - :math:`t_{flow}` is the time of total occupant population to flow through the most restrictive components (time units)
///
/// Args:
///     t_pe1 (float): Pre-evacuation time associated with the first people to respond (time units)
///     t_trav (float): Time taken to traverse the average distance to a place of safety (time units)
///     t_flow (float): Time of total occupant population to flow through the most restrictive components (time units)
///
/// Returns:
///     float: Evacuation time for an enclosure tesc (time units)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_59.equation_59_15.evacuation_time_congestion(30.0, 60.0, 120.0)
///     >>> print(f"{result:.0f} seconds")
fn evacuation_time_congestion(t_pe1: f64, t_trav: f64, t_flow: f64) -> PyResult<f64> {
    Ok(rust_equation_59_15::evacuation_time_congestion(
        t_pe1, t_trav, t_flow,
    ))
}

#[pymodule]
pub fn equation_59_15(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(evacuation_time_congestion, m)?)?;
    Ok(())
}

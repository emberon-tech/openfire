use pyo3::prelude::*;

use openfire::sfpe_handbook::chapter_59::equation_59_16 as rust_equation_59_16;

#[pyfunction]
/// Calculates evacuation time when congestion does not dominate (Equation 59.16).
///
/// In Scenario 2 it is assumed that congestion does not dominate the results;
/// the results are primarily influenced by the time taken to reach safety and
/// the time to respond. The impact of any congestion is dominated by the
/// extensive pre-evacuation phase and the distances that need to be traversed.
///
/// .. math::
///
///    t_{esc} = t_{pe1} + t_{pe99} + t_{trav}
///
/// where:
///
/// - :math:`t_{esc}` is the egress time for an enclosure (time units)
/// - :math:`t_{pe1}` is the pre-evacuation time for the first people to respond, i.e. 1st percentile (time units)
/// - :math:`t_{pe99}` is the pre-evacuation time for the last few occupants to respond, i.e. 99th percentile (time units)
/// - :math:`t_{trav}` is the time taken to traverse the distance to a place of safety (time units)
///
/// Args:
///     t_pe1 (float): Pre-evacuation time for the first people to respond, i.e. 1st percentile (time units)
///     t_pe99 (float): Pre-evacuation time for the last few occupants to respond, i.e. 99th percentile (time units)
///     t_trav (float): Time taken to traverse the distance to a place of safety (time units)
///
/// Returns:
///     float: Egress time for an enclosure tesc (time units)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_59.equation_59_16.evacuation_time_no_congestion(15.0, 180.0, 90.0)
///     >>> print(f"{result:.0f} seconds")
fn evacuation_time_no_congestion(t_pe1: f64, t_pe99: f64, t_trav: f64) -> PyResult<f64> {
    Ok(rust_equation_59_16::evacuation_time_no_congestion(
        t_pe1, t_pe99, t_trav,
    ))
}

#[pymodule]
pub fn equation_59_16(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(evacuation_time_no_congestion, m)?)?;
    Ok(())
}

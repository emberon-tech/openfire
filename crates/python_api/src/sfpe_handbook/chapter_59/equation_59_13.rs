use pyo3::prelude::*;

use openfire::sfpe_handbook::chapter_59::equation_59_13 as rust_equation_59_13;

#[pyfunction]
/// Calculates the specific flow departing from a transition point with two incoming flows (Equation 59.13).
///
/// For cases involving two incoming flows and one outflow from a transition point,
/// such as that which occurs with the merger of a flow down a stair and the
/// entering flow at a floor.
///
/// .. math::
///
///    F_{s(out)} = \frac{F_{s(in1)} \cdot W_{e(in1)} + F_{s(in2)} \cdot W_{e(in2)}}{W_{e(out)}}
///
/// where:
///
/// - :math:`F_{s(out)}` is the specific flow departing from transition point (persons/min/ft or persons/s/m)
/// - :math:`F_{s(in1)}` is the specific flow of first incoming stream (persons/min/ft or persons/s/m)
/// - :math:`W_{e(in1)}` is the effective width of first incoming stream (ft or m)
/// - :math:`F_{s(in2)}` is the specific flow of second incoming stream (persons/min/ft or persons/s/m)
/// - :math:`W_{e(in2)}` is the effective width of second incoming stream (ft or m)
/// - :math:`W_{e(out)}` is the effective width after passing transition point (ft or m)
///
/// Args:
///     fs_in1 (float): Specific flow of first incoming stream (persons/min/ft or persons/s/m)
///     we_in1 (float): Effective width of first incoming stream (ft or m)
///     fs_in2 (float): Specific flow of second incoming stream (persons/min/ft or persons/s/m)
///     we_in2 (float): Effective width of second incoming stream (ft or m)
///     we_out (float): Effective width after passing transition point (ft or m)
///
/// Returns:
///     float: Specific flow departing from transition point Fs(out) (persons/min/ft or persons/s/m)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_59.equation_59_13.specific_flow_out_two_inflows(1.0, 1.0, 1.0, 1.0, 2.0)
///     >>> print(f"{result:.1f} persons/s/m")
fn specific_flow_out_two_inflows(
    fs_in1: f64,
    we_in1: f64,
    fs_in2: f64,
    we_in2: f64,
    we_out: f64,
) -> PyResult<f64> {
    Ok(rust_equation_59_13::specific_flow_out_two_inflows(
        fs_in1, we_in1, fs_in2, we_in2, we_out,
    ))
}

#[pymodule]
pub fn equation_59_13(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(specific_flow_out_two_inflows, m)?)?;
    Ok(())
}

/// Calculates the specific flow departing from a transition point with two incoming flows (Equation 59.13).
///
/// For cases involving two incoming flows and one outflow from a transition point,
/// such as that which occurs with the merger of a flow down a stair and the
/// entering flow at a floor.
///
/// # Arguments
///
/// * `fs_in1` - Specific flow of first incoming stream (persons/min/ft or persons/s/m)
/// * `we_in1` - Effective width of first incoming stream (ft or m)
/// * `fs_in2` - Specific flow of second incoming stream (persons/min/ft or persons/s/m)
/// * `we_in2` - Effective width of second incoming stream (ft or m)
/// * `we_out` - Effective width after passing transition point (ft or m)
///
/// # Returns
///
/// Specific flow departing from transition point Fs(out) (persons/min/ft or persons/s/m)
pub fn specific_flow_out_two_inflows(
    fs_in1: f64,
    we_in1: f64,
    fs_in2: f64,
    we_in2: f64,
    we_out: f64,
) -> f64 {
    (fs_in1 * we_in1 + fs_in2 * we_in2) / we_out
}

#[cfg(not(coverage))]
pub fn specific_flow_out_two_inflows_equation(
    fs_out: String,
    fs_in1: String,
    we_in1: String,
    fs_in2: String,
    we_in2: String,
    we_out: String,
) -> String {
    format!(
        "{} = \\frac{{{} \\cdot {} + {} \\cdot {}}}{{{}}}",
        fs_out, fs_in1, we_in1, fs_in2, we_in2, we_out
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_flow_out_two_inflows_equal() {
        let result = specific_flow_out_two_inflows(1.2, 1.0, 0.8, 1.2, 2.0);
        assert!((result - 1.08).abs() < 1e-6);
    }
}

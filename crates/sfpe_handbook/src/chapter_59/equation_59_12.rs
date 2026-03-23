/// Calculates the specific flow departing from a transition point (Equation 59.12).
///
/// For cases involving one flow into and one flow out of a transition point,
/// this equation determines the specific flow of the route departing from
/// the transition point.
///
/// # Arguments
///
/// * `fs_in` - Specific flow arriving at transition point (persons/min/ft or persons/s/m)
/// * `we_in` - Effective width prior to transition point (ft or m)
/// * `we_out` - Effective width after passing transition point (ft or m)
///
/// # Returns
///
/// Specific flow departing from transition point Fs(out) (persons/min/ft or persons/s/m)
pub fn specific_flow_out(fs_in: f64, we_in: f64, we_out: f64) -> f64 {
    fs_in * we_in / we_out
}

#[cfg(not(coverage))]
pub fn specific_flow_out_equation(
    fs_out: String,
    fs_in: String,
    we_in: String,
    we_out: String,
) -> String {
    format!(
        "{} = \\frac{{{} \\cdot {}}}{{{}}}",
        fs_out, fs_in, we_in, we_out
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_flow_out_narrowing() {
        let result = specific_flow_out(1.0, 2.0, 1.0);
        assert!((result - 2.0).abs() < 1e-6);
    }
}

/// Calculates the flow rate of persons passing a point in an exit route (Equation 59.8).
///
/// The calculated flow is the predicted flow rate of persons passing a particular
/// point in an exit route. This equation assumes the achievable flow rate through
/// a component is directly proportional to its width.
///
/// # Arguments
///
/// * `fs` - Specific flow (persons/min/ft or persons/s/m of effective width)
/// * `we` - Effective width of the component being traversed (ft or m)
///
/// # Returns
///
/// Calculated flow Fc (persons/min or persons/s)
pub fn calculated_flow(fs: f64, we: f64) -> f64 {
    fs * we
}

#[cfg(not(coverage))]
pub fn calculated_flow_equation(fc: String, fs: String, we: String) -> String {
    format!("{} = {} \\cdot {}", fc, fs, we)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculated_flow() {
        // Fs = 1.3 persons/s/m, We = 1.5 m -> Fc = 1.95 persons/s
        let result = calculated_flow(1.3, 1.5);
        assert!((result - 1.95).abs() < 1e-6);
    }
}

/// Calculates specific flow by combining Equations 59.5 and 59.6 (Equation 59.7).
///
/// This equation combines the travel speed equation (59.5) with the specific
/// flow equation (59.6) to express specific flow directly in terms of density.
///
/// # Arguments
///
/// * `a` - Unit conversion constant (0.266 for SI units, 2.86 for imperial units)
/// * `d` - Population density in persons per unit area
/// * `k` - Speed constant from Table 59.2
///
/// # Returns
///
/// Specific flow Fs (persons/min/ft or persons/s/m of effective width)
pub fn specific_flow(a: f64, d: f64, k: f64) -> f64 {
    (1.0 - a * d) * k * d
}

#[cfg(not(coverage))]
pub fn specific_flow_equation(fs: String, a: String, d: String, k: String) -> String {
    format!(
        "{} = (1 - {} \\cdot {}) \\cdot {} \\cdot {}",
        fs, a, d, k, d
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_flow_combined() {
        let result = specific_flow(0.266, 0.5, 1.4);
        assert!((result - 0.6069).abs() < 1e-6);
    }
}

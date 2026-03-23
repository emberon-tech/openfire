/// Calculates flow rate by combining Equations 59.7 and 59.8 (Equation 59.9).
///
/// This equation combines the specific flow equation (59.7) with the calculated
/// flow equation (59.8) to express flow rate directly in terms of the fundamental
/// parameters.
///
/// # Arguments
///
/// * `a` - Unit conversion constant (0.266 for SI units, 2.86 for imperial units)
/// * `d` - Population density in persons per unit area
/// * `k` - Speed constant from Table 59.2
/// * `we` - Effective width of the component being traversed
///
/// # Returns
///
/// Calculated flow Fc (persons/min when using imperial units, persons/s when using SI units)
pub fn calculated_flow(a: f64, d: f64, k: f64, we: f64) -> f64 {
    (1.0 - a * d) * k * d * we
}

#[cfg(not(coverage))]
pub fn calculated_flow_equation(fc: String, a: String, d: String, k: String, we: String) -> String {
    format!(
        "{} = (1 - {} \\cdot {}) \\cdot {} \\cdot {} \\cdot {}",
        fc, a, d, k, d, we
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculated_flow() {
        // Using SI units: a = 0.266, D = 0.5 persons/m², k = 1.40 m/s, We = 1.2 m
        // Fc = (1 - 0.266 * 0.5) * 1.40 * 0.5 * 1.2 = 0.867 * 0.84 = 0.72828
        let result = calculated_flow(0.266, 0.5, 1.40, 1.2);
        assert!((result - 0.72828).abs() < 1e-5);
    }

    #[test]
    fn test_calculated_flow_zero_density() {
        // When D = 0, Fc = 0
        let result = calculated_flow(0.266, 0.0, 1.40, 1.2);
        assert!((result - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_calculated_flow_zero_width() {
        // When We = 0, Fc = 0
        let result = calculated_flow(0.266, 0.5, 1.40, 0.0);
        assert!((result - 0.0).abs() < 1e-6);
    }
}

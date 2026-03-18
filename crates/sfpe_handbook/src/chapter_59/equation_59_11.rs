/// Calculates the time for passage combining Equations 59.9 and 59.10 (Equation 59.11).
///
/// This equation combines the calculated flow equation (59.9) with the time for
/// passage equation (59.10) to express time directly in terms of the fundamental
/// parameters.
///
/// # Arguments
///
/// * `p` - Population size (persons)
/// * `a` - Unit conversion constant (0.266 for SI units, 2.86 for imperial units)
/// * `d` - Population density in persons per unit area
/// * `k` - Speed constant from Table 59.2
/// * `we` - Effective width of the component being traversed
///
/// # Returns
///
/// Time for passage tp (minutes when using imperial units, seconds when using SI units)
pub fn time_for_passage(p: f64, a: f64, d: f64, k: f64, we: f64) -> f64 {
    p / ((1.0 - a * d) * k * d * we)
}

#[cfg(not(coverage))]
pub fn time_for_passage_equation(
    tp: String,
    p: String,
    a: String,
    d: String,
    k: String,
    we: String,
) -> String {
    format!(
        "{} = \\frac{{{}}}{{(1 - {} \\cdot {}) \\cdot {} \\cdot {} \\cdot {}}}",
        tp, p, a, d, k, d, we
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_for_passage() {
        let result = time_for_passage(100.0, 0.266, 0.5, 1.40, 1.2);
        assert!((result - 137.309825891).abs() < 1e-6);
    }
}

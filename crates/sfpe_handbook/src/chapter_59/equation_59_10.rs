/// Calculates the time for passage (Equation 59.10).
///
/// The time for passage is the time for a group of persons to pass a point
/// in an exit route.
///
/// # Arguments
///
/// * `p` - Population size (persons)
/// * `fc` - Calculated flow (persons/min or persons/s)
///
/// # Returns
///
/// Time for passage tp (minutes when Fc is in persons/min; seconds when Fc is in persons/s)
pub fn time_for_passage(p: f64, fc: f64) -> f64 {
    p / fc
}

#[cfg(not(coverage))]
pub fn time_for_passage_equation(tp: String, p: String, fc: String) -> String {
    format!("{} = \\frac{{{}}}{{{}}}", tp, p, fc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_for_passage() {
        let result = time_for_passage(100.0, 1.0);
        assert!((result - 100.0).abs() < 1e-6);
    }
}

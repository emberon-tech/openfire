/// Calculates pedestrian travel speed along a path (Equation 59.5).
///
/// # Arguments
///
/// * `k` - Speed constant from Table 59.2 (units depend on unit system)
/// * `a` - Unit conversion constant (0.266 for SI, 2.86 for imperial)
/// * `d` - Population density in persons per unit area
///
/// # Returns
///
/// Speed S along the line of travel
pub fn travel_speed(k: f64, a: f64, d: f64) -> f64 {
    k - a * k * d
}

#[cfg(not(coverage))]
pub fn travel_speed_equation(s: String, k: String, a: String, d: String) -> String {
    format!("{} = {} - {} \\cdot {} \\cdot {}", s, k, a, k, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_travel_speed_generic() {
        let result = travel_speed(1.4, 0.266, 0.7);
        assert!((result - 1.13932).abs() < 1e-6);
    }
}

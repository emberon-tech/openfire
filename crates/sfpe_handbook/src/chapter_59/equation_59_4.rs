pub fn speed(k: f64, a: f64, d: f64) -> f64 {
    k - a * k * d
}

#[cfg(not(coverage))]
pub fn speed_equation(s: String, k: String, a: String, d: String) -> String {
    format!("{} = {} - {} \\cdot {} \\cdot {}", s, k, a, k, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speed() {
        let result = speed(1.4, 0.266, 2.0);
        let expected = 0.6552;
        assert!((result - expected).abs() < 1e-6);
    }
}

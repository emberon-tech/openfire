/// Calculates specific flow of evacuating persons (Equation 59.6).
///
/// Specific flow is the flow of evacuating persons past a point in the exit
/// route per unit of time per unit of effective width.
///
/// # Arguments
///
/// * `s` - Speed along the line of travel (from Equation 59.5)
/// * `d` - Population density in persons per unit area
///
/// # Returns
///
/// Specific flow Fs (persons/min/ft or persons/s/m of effective width)
pub fn specific_flow(s: f64, d: f64) -> f64 {
    s * d
}

#[cfg(not(coverage))]
pub fn specific_flow_equation(fs: String, s: String, d: String) -> String {
    format!("{} = {} \\cdot {}", fs, s, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_flow() {
        // S = 1.0 m/s, D = 0.5 persons/m² -> Fs = 0.5 persons/s/m
        let result = specific_flow(1.0, 0.5);
        assert!((result - 0.5).abs() < 1e-6);
    }
}

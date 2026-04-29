const G: f64 = 9.81;

pub fn terminal_velocity(diameter: f64, drag_coefficient: f64, rho_w: f64, rho_a: f64) -> f64 {
    let numerator = 4.0 * G * diameter * (rho_w - rho_a);
    let denominator = 3.0 * drag_coefficient * rho_a;
    (numerator / denominator).sqrt()
}

#[cfg(not(coverage))]
pub fn terminal_velocity_equation(
    u_t: String,
    diameter: String,
    drag_coefficient: String,
    rho_w: String,
    rho_a: String,
) -> String {
    format!(
        "{} = \\sqrt{{\\dfrac{{4 \\cdot g \\cdot {} \\cdot ({} - {})}}{{3 \\cdot {} \\cdot {}}}}}",
        u_t, diameter, rho_w, rho_a, drag_coefficient, rho_a
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typical_water_droplet() {
        let diameter = 0.001;
        let drag_coefficient = 0.5;
        let rho_w = 1000.0;
        let rho_a = 1.2;
        let result = terminal_velocity(diameter, drag_coefficient, rho_w, rho_a);
        let expected = (4.0 * G * diameter * (rho_w - rho_a) / (3.0 * drag_coefficient * rho_a))
            .sqrt();
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_velocity_increases_with_diameter() {
        let drag_coefficient = 0.5;
        let rho_w = 1000.0;
        let rho_a = 1.2;
        let v_small = terminal_velocity(0.0005, drag_coefficient, rho_w, rho_a);
        let v_large = terminal_velocity(0.002, drag_coefficient, rho_w, rho_a);
        assert!(v_large > v_small);
    }

    #[test]
    fn test_velocity_decreases_with_drag() {
        let diameter = 0.001;
        let rho_w = 1000.0;
        let rho_a = 1.2;
        let v_low_drag = terminal_velocity(diameter, 0.4, rho_w, rho_a);
        let v_high_drag = terminal_velocity(diameter, 1.5, rho_w, rho_a);
        assert!(v_high_drag < v_low_drag);
    }

    #[test]
    fn test_zero_density_difference() {
        let result = terminal_velocity(0.001, 0.5, 1.2, 1.2);
        assert_eq!(result, 0.0);
    }
}

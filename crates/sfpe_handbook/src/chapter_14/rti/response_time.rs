pub fn activation_time(rti: f64, u: f64, t_g: f64, t_a: f64, t_act: f64) -> f64 {
    if t_g <= t_act {
        return f64::INFINITY;
    }
    (rti / u.sqrt()) * ((t_g - t_a) / (t_g - t_act)).ln()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_activation() {
        let rti = 100.0;
        let u = 1.0;
        let t_g = 200.0;
        let t_a = 20.0;
        let t_act = 68.0;
        let result = activation_time(rti, u, t_g, t_a, t_act);
        let expected = (rti / u.sqrt()) * ((t_g - t_a) / (t_g - t_act)).ln();
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_no_activation_when_gas_below_setpoint() {
        let result = activation_time(100.0, 1.0, 60.0, 20.0, 68.0);
        assert!(result.is_infinite() && result.is_sign_positive());
    }

    #[test]
    fn test_no_activation_when_gas_equal_setpoint() {
        let result = activation_time(100.0, 1.0, 68.0, 20.0, 68.0);
        assert!(result.is_infinite() && result.is_sign_positive());
    }

    #[test]
    fn test_higher_rti_gives_longer_response() {
        let u = 2.0;
        let t_g = 200.0;
        let t_a = 20.0;
        let t_act = 68.0;
        let t_fast = activation_time(50.0, u, t_g, t_a, t_act);
        let t_slow = activation_time(250.0, u, t_g, t_a, t_act);
        assert!(t_slow > t_fast);
    }

    #[test]
    fn test_higher_velocity_gives_shorter_response() {
        let rti = 100.0;
        let t_g = 200.0;
        let t_a = 20.0;
        let t_act = 68.0;
        let t_low_u = activation_time(rti, 0.5, t_g, t_a, t_act);
        let t_high_u = activation_time(rti, 4.0, t_g, t_a, t_act);
        assert!(t_high_u < t_low_u);
    }
}

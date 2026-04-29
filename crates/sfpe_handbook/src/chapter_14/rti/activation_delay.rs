use crate::chapter_14::alpert::temperature::temperature_rise;
use crate::chapter_14::alpert::velocity::velocity;
use crate::chapter_14::rti::response_time::activation_time;

pub fn transport_time(q: f64, height: f64, radial_position: f64) -> f64 {
    let u = velocity(q, height, radial_position);
    if u == 0.0 {
        return f64::INFINITY;
    }
    radial_position / u
}

pub fn total_activation_delay(
    rti: f64,
    q: f64,
    height: f64,
    radial_position: f64,
    t_a: f64,
    t_act: f64,
) -> f64 {
    let u = velocity(q, height, radial_position);
    let delta_t = temperature_rise(q, height, radial_position);
    let t_g = t_a + delta_t;
    let t_thermal = activation_time(rti, u, t_g, t_a, t_act);
    let t_transport = if u == 0.0 {
        f64::INFINITY
    } else {
        radial_position / u
    };
    t_transport + t_thermal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_time_basic() {
        let q = 1000.0;
        let height = 4.0;
        let radial_position = 3.0;
        let u = velocity(q, height, radial_position);
        let result = transport_time(q, height, radial_position);
        assert!((result - radial_position / u).abs() < 1e-6);
    }

    #[test]
    fn test_transport_time_zero_radius() {
        let result = transport_time(1000.0, 4.0, 0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_total_delay_matches_components() {
        let rti = 100.0;
        let q = 2000.0;
        let height = 5.0;
        let radial_position = 3.0;
        let t_a = 20.0;
        let t_act = 68.0;

        let u = velocity(q, height, radial_position);
        let delta_t = temperature_rise(q, height, radial_position);
        let t_g = t_a + delta_t;
        let t_thermal = activation_time(rti, u, t_g, t_a, t_act);
        let t_transport = radial_position / u;
        let expected = t_transport + t_thermal;

        let result = total_activation_delay(rti, q, height, radial_position, t_a, t_act);
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_total_delay_no_activation_when_jet_too_cool() {
        let rti = 100.0;
        let q = 50.0;
        let height = 10.0;
        let radial_position = 8.0;
        let t_a = 20.0;
        let t_act = 200.0;
        let result = total_activation_delay(rti, q, height, radial_position, t_a, t_act);
        assert!(result.is_infinite() && result.is_sign_positive());
    }

    #[test]
    fn test_total_delay_increases_with_radius() {
        let rti = 100.0;
        let q = 5000.0;
        let height = 6.0;
        let t_a = 20.0;
        let t_act = 68.0;
        let near = total_activation_delay(rti, q, height, 1.0, t_a, t_act);
        let far = total_activation_delay(rti, q, height, 4.0, t_a, t_act);
        assert!(far > near);
    }
}

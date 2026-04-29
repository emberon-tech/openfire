pub fn temperature_rise(q: f64, height: f64, radial_position: f64) -> f64 {
    if radial_position / height <= 0.18 {
        16.9 * q.powf(2.0 / 3.0) / height.powf(5.0 / 3.0)
    } else {
        5.38 * (q / radial_position).powf(2.0 / 3.0) / height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chapter_14::alpert::heat_release::from_temperature_and_position;

    #[test]
    fn test_zero_radial_position() {
        let q = 1000.0;
        let height = 10.0;
        let radial_position = 0.0;
        let result = temperature_rise(q, height, radial_position);
        let expected = 16.9 * q.powf(2.0 / 3.0) / height.powf(5.0 / 3.0);
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_radial_position_at_threshold() {
        let q = 1000.0;
        let height = 10.0;
        let radial_position = 0.18 * height;
        let result = temperature_rise(q, height, radial_position);
        let expected = 16.9 * q.powf(2.0 / 3.0) / height.powf(5.0 / 3.0);
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_radial_position_greater_than_threshold() {
        let q = 1000.0;
        let height = 10.0;
        let radial_position = 5.0;
        let result = temperature_rise(q, height, radial_position);
        let expected = 5.38 * (q / radial_position).powf(2.0 / 3.0) / height;
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_inverse_consistency_near_field() {
        let q = 1500.0;
        let height = 8.0;
        let radial_position = 0.5;
        let temp_amb = 293.0;
        let delta_t = temperature_rise(q, height, radial_position);
        let recovered_q =
            from_temperature_and_position(temp_amb + delta_t, temp_amb, height, radial_position);
        assert!((recovered_q - q).abs() < 1e-3);
    }

    #[test]
    fn test_inverse_consistency_far_field() {
        let q = 2500.0;
        let height = 6.0;
        let radial_position = 4.0;
        let temp_amb = 293.0;
        let delta_t = temperature_rise(q, height, radial_position);
        let recovered_q =
            from_temperature_and_position(temp_amb + delta_t, temp_amb, height, radial_position);
        assert!((recovered_q - q).abs() < 1e-3);
    }

    #[test]
    fn test_zero_heat_release() {
        let result = temperature_rise(0.0, 10.0, 1.0);
        assert_eq!(result, 0.0);
    }
}

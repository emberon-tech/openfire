pub fn velocity(q: f64, height: f64, radial_position: f64) -> f64 {
    if radial_position / height <= 0.15 {
        0.96 * (q / height).powf(1.0 / 3.0)
    } else {
        0.195 * q.powf(1.0 / 3.0) * height.powf(0.5) / radial_position.powf(5.0 / 6.0)
    }
}

#[cfg(not(coverage))]
pub fn velocity_equation(
    u: String,
    q: String,
    height: String,
    radial_position: String,
) -> String {
    format!(
        "{} = \\begin{{cases}} 0.96 \\cdot \\left(\\dfrac{{{}}}{{{}}}\\right)^{{1/3}} & \\text{{if }} \\dfrac{{{}}}{{{}}} \\leq 0.15 \\\\[6pt] 0.195 \\cdot \\dfrac{{{}^{{1/3}} \\cdot {}^{{1/2}}}}{{{}^{{5/6}}}} & \\text{{if }} \\dfrac{{{}}}{{{}}} > 0.15 \\end{{cases}}",
        u,
        q, height,
        radial_position, height,
        q, height, radial_position,
        radial_position, height
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_radial_position() {
        let q = 1000.0;
        let height = 10.0;
        let radial_position = 0.0;
        let result = velocity(q, height, radial_position);
        let expected = 0.96 * (q / height).powf(1.0 / 3.0);
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_radial_position_at_threshold() {
        let q = 1000.0;
        let height = 10.0;
        let radial_position = 0.15 * height;
        let result = velocity(q, height, radial_position);
        let expected = 0.96 * (q / height).powf(1.0 / 3.0);
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_radial_position_greater_than_threshold() {
        let q = 1000.0;
        let height = 10.0;
        let radial_position = 5.0;
        let result = velocity(q, height, radial_position);
        let expected =
            0.195 * q.powf(1.0 / 3.0) * height.powf(0.5) / radial_position.powf(5.0 / 6.0);
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_velocity_decreases_with_radius_in_far_field() {
        let q = 2000.0;
        let height = 8.0;
        let v_near = velocity(q, height, 2.0);
        let v_far = velocity(q, height, 6.0);
        assert!(v_far < v_near);
    }

    #[test]
    fn test_zero_heat_release() {
        let result = velocity(0.0, 10.0, 1.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_velocity_increases_with_heat_release() {
        let height = 10.0;
        let radial_position = 2.0;
        let v_low = velocity(500.0, height, radial_position);
        let v_high = velocity(2000.0, height, radial_position);
        assert!(v_high > v_low);
    }
}

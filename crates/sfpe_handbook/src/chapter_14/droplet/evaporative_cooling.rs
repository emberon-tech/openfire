pub fn cooling_power(m_dot: f64, c_p_w: f64, t_boil: f64, t_w: f64, h_fg: f64) -> f64 {
    m_dot * (c_p_w * (t_boil - t_w) + h_fg)
}

#[cfg(not(coverage))]
pub fn cooling_power_equation(
    q: String,
    m_dot: String,
    c_p_w: String,
    t_boil: String,
    t_w: String,
    h_fg: String,
) -> String {
    format!(
        "{} = {} \\cdot \\left({} \\cdot ({} - {}) + {}\\right)",
        q, m_dot, c_p_w, t_boil, t_w, h_fg
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typical_values() {
        let m_dot = 0.001;
        let c_p_w = 4186.0;
        let t_boil = 100.0;
        let t_w = 20.0;
        let h_fg = 2_257_000.0;
        let result = cooling_power(m_dot, c_p_w, t_boil, t_w, h_fg);
        let expected = m_dot * (c_p_w * (t_boil - t_w) + h_fg);
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_cooling_scales_linearly_with_mass_flow() {
        let c_p_w = 4186.0;
        let t_boil = 100.0;
        let t_w = 20.0;
        let h_fg = 2_257_000.0;
        let q_low = cooling_power(0.001, c_p_w, t_boil, t_w, h_fg);
        let q_high = cooling_power(0.002, c_p_w, t_boil, t_w, h_fg);
        assert!((q_high - 2.0 * q_low).abs() < 1e-6);
    }

    #[test]
    fn test_water_at_boiling_only_uses_latent() {
        let m_dot = 0.001;
        let c_p_w = 4186.0;
        let t_boil = 100.0;
        let h_fg = 2_257_000.0;
        let result = cooling_power(m_dot, c_p_w, t_boil, t_boil, h_fg);
        assert!((result - m_dot * h_fg).abs() < 1e-6);
    }

    #[test]
    fn test_zero_mass_flow() {
        let result = cooling_power(0.0, 4186.0, 100.0, 20.0, 2_257_000.0);
        assert_eq!(result, 0.0);
    }
}

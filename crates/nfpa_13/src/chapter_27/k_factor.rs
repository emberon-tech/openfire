pub fn flow_rate(k: f64, pressure: f64) -> f64 {
    k * pressure.sqrt()
}

pub fn pressure(k: f64, flow_rate: f64) -> f64 {
    (flow_rate / k).powi(2)
}

pub fn k_factor(flow_rate: f64, pressure: f64) -> f64 {
    flow_rate / pressure.sqrt()
}

#[cfg(not(coverage))]
pub fn flow_rate_equation(q: String, k: String, p: String) -> String {
    format!("{} = {} \\cdot \\sqrt{{{}}}", q, k, p)
}

#[cfg(not(coverage))]
pub fn pressure_equation(p: String, q: String, k: String) -> String {
    format!("{} = \\left(\\dfrac{{{}}}{{{}}}\\right)^{{2}}", p, q, k)
}

#[cfg(not(coverage))]
pub fn k_factor_equation(k: String, q: String, p: String) -> String {
    format!("{} = \\dfrac{{{}}}{{\\sqrt{{{}}}}}", k, q, p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_rate_basic() {
        let k = 80.0;
        let p = 1.0;
        assert!((flow_rate(k, p) - 80.0).abs() < 1e-9);
    }

    #[test]
    fn test_flow_rate_quadruple_pressure_doubles_flow() {
        let k = 80.0;
        let q1 = flow_rate(k, 1.0);
        let q4 = flow_rate(k, 4.0);
        assert!((q4 - 2.0 * q1).abs() < 1e-9);
    }

    #[test]
    fn test_pressure_inverts_flow_rate() {
        let k = 115.0;
        let p_in = 2.5;
        let q = flow_rate(k, p_in);
        let p_out = pressure(k, q);
        assert!((p_out - p_in).abs() < 1e-9);
    }

    #[test]
    fn test_k_factor_inverts_flow_rate() {
        let k_in = 100.0;
        let p = 3.0;
        let q = flow_rate(k_in, p);
        let k_out = k_factor(q, p);
        assert!((k_out - k_in).abs() < 1e-9);
    }

    #[test]
    fn test_zero_pressure_gives_zero_flow() {
        assert_eq!(flow_rate(80.0, 0.0), 0.0);
    }
}

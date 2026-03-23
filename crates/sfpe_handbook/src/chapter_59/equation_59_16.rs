/// Calculates evacuation time when congestion does not dominate (Equation 59.16).
///
/// In Scenario 2 it is assumed that congestion does not dominate the results;
/// the results are primarily influenced by the time taken to reach safety (ttrav)
/// and the time to respond (tpe). This scenario is not necessarily based on the
/// assumption that congestion does not develop; only that the impact of the
/// congestion is dominated by the extensive pre-evacuation phase and the distances
/// that need to be traversed, and is not a factor in the calculation.
///
/// # Arguments
///
/// * `t_pe1` - Pre-evacuation time for the first people to respond, i.e. 1st percentile (time units)
/// * `t_pe99` - Pre-evacuation time for the last few occupants to respond, i.e. 99th percentile (time units)
/// * `t_trav` - Time taken to traverse the distance to a place of safety (time units)
///
/// # Returns
///
/// Egress time for an enclosure tesc (time units)
pub fn evacuation_time_no_congestion(t_pe1: f64, t_pe99: f64, t_trav: f64) -> f64 {
    t_pe1 + t_pe99 + t_trav
}

#[cfg(not(coverage))]
pub fn evacuation_time_no_congestion_equation(
    t_esc: String,
    t_pe1: String,
    t_pe99: String,
    t_trav: String,
) -> String {
    format!("{} = {} + {} + {}", t_esc, t_pe1, t_pe99, t_trav)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evacuation_time_no_congestion() {
        let result = evacuation_time_no_congestion(15.0, 180.0, 90.0);
        assert!((result - 285.0).abs() < 1e-6);
    }
}

/// Calculates evacuation time when congestion dominates (Equation 59.15).
///
/// In Scenario 1 it is assumed that congestion dominates the results produced.
/// In such situations the time required for the evacuation of an enclosure
/// depends on the pre-evacuation time and unrestricted walking time of the
/// first few occupants to start to leave; these determine the time for
/// congestion to develop. Once queues have formed at the "constraining"
/// component, the time to clear the building becomes a function of the
/// number of occupants and the evacuation flow rate capacity of these components.
///
/// # Arguments
///
/// * `t_pe1` - Pre-evacuation time associated with the first people to respond (time units)
/// * `t_trav` - Time taken to traverse the average distance to a place of safety (time units)
/// * `t_flow` - Time of total occupant population to flow through the most restrictive components (time units)
///
/// # Returns
///
/// Evacuation time for an enclosure tesc (time units)
pub fn evacuation_time_congestion(t_pe1: f64, t_trav: f64, t_flow: f64) -> f64 {
    t_pe1 + t_trav + t_flow
}

#[cfg(not(coverage))]
pub fn evacuation_time_congestion_equation(
    t_esc: String,
    t_pe1: String,
    t_trav: String,
    t_flow: String,
) -> String {
    format!("{} = {} + {} + {}", t_esc, t_pe1, t_trav, t_flow)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evacuation_time_congestion() {
        let result = evacuation_time_congestion(30.0, 60.0, 120.0);
        assert!((result - 210.0).abs() < 1e-6);
    }
}

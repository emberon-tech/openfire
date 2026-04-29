#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HazardClass {
    LightHazard,
    OrdinaryHazardGroup1,
    OrdinaryHazardGroup2,
    ExtraHazardGroup1,
    ExtraHazardGroup2,
}

pub fn coverage_area(spacing: f64, branchline_distance: f64) -> f64 {
    spacing * branchline_distance
}

pub fn max_coverage_area(hazard: HazardClass) -> f64 {
    match hazard {
        HazardClass::LightHazard => 20.9,
        HazardClass::OrdinaryHazardGroup1 => 12.1,
        HazardClass::OrdinaryHazardGroup2 => 12.1,
        HazardClass::ExtraHazardGroup1 => 9.3,
        HazardClass::ExtraHazardGroup2 => 9.3,
    }
}

pub fn is_within_limit(spacing: f64, branchline_distance: f64, hazard: HazardClass) -> bool {
    coverage_area(spacing, branchline_distance) <= max_coverage_area(hazard)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coverage_area_basic() {
        assert!((coverage_area(4.0, 3.0) - 12.0).abs() < 1e-9);
    }

    #[test]
    fn test_coverage_area_zero() {
        assert_eq!(coverage_area(0.0, 4.0), 0.0);
    }

    #[test]
    fn test_max_coverage_light() {
        assert_eq!(max_coverage_area(HazardClass::LightHazard), 20.9);
    }

    #[test]
    fn test_max_coverage_ordinary_groups_equal() {
        assert_eq!(
            max_coverage_area(HazardClass::OrdinaryHazardGroup1),
            max_coverage_area(HazardClass::OrdinaryHazardGroup2)
        );
    }

    #[test]
    fn test_max_coverage_extra_groups_equal() {
        assert_eq!(
            max_coverage_area(HazardClass::ExtraHazardGroup1),
            max_coverage_area(HazardClass::ExtraHazardGroup2)
        );
    }

    #[test]
    fn test_max_coverage_decreases_with_hazard() {
        let light = max_coverage_area(HazardClass::LightHazard);
        let ordinary = max_coverage_area(HazardClass::OrdinaryHazardGroup1);
        let extra = max_coverage_area(HazardClass::ExtraHazardGroup1);
        assert!(light > ordinary);
        assert!(ordinary > extra);
    }

    #[test]
    fn test_within_limit_passes() {
        assert!(is_within_limit(4.0, 3.0, HazardClass::LightHazard));
    }

    #[test]
    fn test_within_limit_fails() {
        assert!(!is_within_limit(4.0, 4.0, HazardClass::ExtraHazardGroup1));
    }

    #[test]
    fn test_within_limit_at_boundary() {
        let area = max_coverage_area(HazardClass::OrdinaryHazardGroup1);
        let s = area.sqrt();
        assert!(is_within_limit(s, s, HazardClass::OrdinaryHazardGroup1));
    }
}

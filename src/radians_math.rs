use crate::PI_F32;

//const 𝝅: f32= std::f32::consts::PI;
pub(crate) const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

/// Normalize to be between 0 and 2 PI
pub fn norm_rad(angle_in_radians: f32) -> f32 {
    let angle_in_radians = angle_in_radians % TWO_PI;
    if angle_in_radians < 0.0 {
        TWO_PI + angle_in_radians
    } else {
        angle_in_radians
    }
}

// same as rad_wrap_diff
pub fn clockwise_diff(low: f32, high: f32) -> f32 {
    let diff = high - low;
    if low > high {
        TWO_PI + diff
    } else {
        diff
    }
}

/// Difference between two values in a wrapping radians.
pub fn rad_wrap_diff(low: f32, high: f32) -> f32 {
    if low <= high {
        high - low
    } else {
        2.0 * PI_F32 + high - low
    }
}

/// Wraps a value between -PI and +PI
pub fn rad_wrap(val: f32) -> f32 {
    if ((-std::f32::consts::PI)..=(std::f32::consts::PI)).contains(&val) {
        val
    } else {
        val.rem_euclid(std::f32::consts::PI)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;
    use pretty_assertions::{assert_eq /*assert_ne*/};

    const FLOAT_SAMENESS_MARGIN: f32 = 0.00001;

    fn float_same(val: f32, cmp: f32) -> bool {
        (val - cmp).abs() < FLOAT_SAMENESS_MARGIN
    }

    #[test]
    fn test_rad_wrap() {
        assert!(float_same(rad_wrap(1.0), 1.0));
        assert!(float_same(rad_wrap(4.0), 4.0 - PI_F32));
        //assert!(float_same(rad_wrap(-4.0), 4.0 - PI_F32));
    }

    #[test]
    fn solve_rad_wrap_diff() {
        assert!(float_same(rad_wrap_diff(2., 2.), 0.));
        assert!(float_same(rad_wrap_diff(-PI_F32, PI_F32), 2.0 * PI_F32));
        assert!(float_same(rad_wrap_diff(PI_F32 - 0.1, -PI_F32), 0.1));
    }

    #[test]
    fn test_norm_rad() {
        assert_eq!(norm_rad(1.0), 1.0);
        assert_f32_near!(norm_rad(4.0), 4.0);
        assert_f32_near!(norm_rad(TWO_PI + 4.0), 4.0);
        assert_eq!(norm_rad(-2.0), TWO_PI - 2.0);
        //assert!(float_same(rad_wrap(-4.0), 4.0 - PI_F32));
    }
}

use std::f32::consts::PI;

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

/// Difference between two values in a wrapping radians. Can
/// produce values between -2𝝅 .. -2𝝅 .
pub fn clockwise_diff(low: f32, high: f32) -> f32 {
    let diff = high - low;
    if low > high {
        TWO_PI + diff
    } else {
        diff
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
    use std::f32::consts::FRAC_PI_4;

    use super::*;
    use assert_float_eq::*;
    use pretty_assertions::{assert_eq /*assert_ne*/};

    const FLOAT_SAMENESS_MARGIN: f32 = 0.00001;

    fn float_same(val: f32, cmp: f32) -> bool {
        (val - cmp).abs() < FLOAT_SAMENESS_MARGIN
    }

    #[test]
    fn test_rad_wrap() {
        assert_f32_near!(rad_wrap(1.0), 1.0);
        assert_f32_near!(rad_wrap(4.0), 4.0 - PI);
    }

    #[test]
    #[test]
    fn test_clockwise_diff() {
        assert_f32_near!(clockwise_diff(-FRAC_PI_4, 0.0), FRAC_PI_4);
        assert_f32_near!(clockwise_diff(-FRAC_PI_4, 7.0 * FRAC_PI_4), TWO_PI);
        assert_f32_near!(clockwise_diff(-FRAC_PI_4, 1.0), FRAC_PI_4 + 1.0);
        assert_f32_near!(clockwise_diff(2., 2.), 0.0);
        assert_f32_near!(clockwise_diff(-PI, PI), 2.0 * PI);
        assert_f32_near!(clockwise_diff(PI - 0.1, -PI), 0.1);
    }

    #[test]
    fn test_norm_rad() {
        assert_eq!(norm_rad(1.0), 1.0);
        assert_f32_near!(norm_rad(4.0), 4.0);
        assert_f32_near!(norm_rad(TWO_PI + 4.0), 4.0);
        assert_eq!(norm_rad(-2.0), TWO_PI - 2.0);
    }
}

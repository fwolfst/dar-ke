use crate::PI_F32;

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
}

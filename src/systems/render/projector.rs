use crate::{
    radians_math::{clockwise_diff, TWO_PI},
    HALF_VIEW_ANGLE, RENDER_WIDTH, VIEW_ANGLE,
};

/// A projector, stores some intermediate calculations
/// for faster processing. Might become a renderer.
pub struct Projector {
    left_view_rad: f32,
    pixel_per_rad: f32,
}

/// Generate a projector
pub fn make_projector(view_direction_rad: f32) -> Projector {
    Projector {
        left_view_rad: view_direction_rad - HALF_VIEW_ANGLE,
        pixel_per_rad: (RENDER_WIDTH as f32) / VIEW_ANGLE,
    }
}

// TODO these could Result<> (offscreen)
impl Projector {
    /// Radians projected to screen space
    pub fn screen_x_of_rad(&self, rad: f32) -> i32 {
        let k = clockwise_diff(self.left_view_rad, rad);
        // normalize
        let k = if k > (2.0 * std::f32::consts::PI) {
            k - 2.0 * std::f32::consts::PI
        } else {
            k
        };
        (k * self.pixel_per_rad).round() as i32
    }

    /// Gives coordinates for an angle both "left" and "right" of the
    /// view direction.
    /// For an object not on screen, one can ask how far the projected
    /// point is towards the left and also towards right.
    ///
    /// ```
    /// let p = Projector.new(0.0);
    /// assert_eq(p.screen_x2_of_rad(3.1), (12,20));
    /// ```
    pub fn screen_x2_of_rad(&self, rad: f32) -> (i32, i32) {
        let k = clockwise_diff(self.left_view_rad, rad);

        (
            (k * self.pixel_per_rad).round() as i32,
            -((TWO_PI - k) * self.pixel_per_rad).round() as i32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq /*assert_ne*/};

    // TODO This is bound to assumptions regarding view angle and focal
    // depth. We want these to be baked in for ultra speed, at the same time
    // I want to be able to play with it.
    #[test]
    fn projection_from_radians() {
        let p = make_projector(0.0);

        assert_eq!(p.screen_x_of_rad(0.0), (RENDER_WIDTH / 2) as i32);

        assert_eq!(
            p.screen_x_of_rad(std::f32::consts::FRAC_PI_4),
            (RENDER_WIDTH) as i32
        );

        assert_eq!(p.screen_x_of_rad(-std::f32::consts::FRAC_PI_4), 0);
    }
}

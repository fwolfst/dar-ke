use crate::{HALF_VIEW_ANGLE, RENDER_WIDTH, VIEW_ANGLE};

// A projector, stores some intermediate calculations
// for faster processing. Might become a renderer.
pub struct Projector {
    view_direction_rad: f32,
    left_view_rad: f32,
    pixel_per_rad: f32,
}

pub fn make_projector(view_direction_rad: f32) -> Projector {
    Projector {
        view_direction_rad,
        left_view_rad: view_direction_rad - HALF_VIEW_ANGLE,
        pixel_per_rad: (RENDER_WIDTH as f32) / VIEW_ANGLE,
    }
}

impl Projector {
    pub fn screen_x_of_rad(self, rad: f32) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq /*assert_ne*/};

    #[test]
    fn projection_from_radians() {
        let p = make_projector(0.0);
        assert_eq!(p.screen_x_of_rad(0.0), 46);
    }
}

use crate::components::AtHorizon;
use crate::components::Giant;
use crate::components::Light;
use crate::components::Params;
use crate::radians_math::*;
use crate::{components::Player, HALF_VIEW_ANGLE, RENDER_HEIGHT, RENDER_WIDTH, VIEW_ANGLE};
use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;

const HORIZON_COL: [u8; 3] = [1, 2, 3];

pub fn render(
    mut pb: QueryPixelBuffer,
    player: Query<&Player>,
    giants: Query<&Giant>,
    lights: Query<(&Light, &AtHorizon)>,
    params: Res<Params>,
) {
    let player = player.single();
    let mut frame = pb.frame();
    let horizon = frame.size().y / 2 + player.height as u32;

    //let mut red_mult = 0.5 * (1.0 - player.direction);
    //let mut green_mult = 0.5 * (-1.0 + player.direction);
    //if params.sky_dir_up {
    //    red_mult = 1.0 / red_mult;
    //    green_mult = 1.0 / green_mult;
    //}

    draw_sky(horizon as u8, &mut frame, params.sky_up_bright);

    draw_ground(horizon, &mut frame, params.ground_up_bright);

    for (light, at_horizon) in &lights {
        let pos_of_obj_screen = project_x(player.direction, at_horizon.angle);
        frame
            .set(
                UVec2::new(pos_of_obj_screen as u32, horizon - 1),
                light.color,
            )
            .ok();
    }

    // Draw poles
    // N = black, S = red, E = green, W = yellow
    let north = project_x(player.direction, 0.0);
    if north > 0.0 && north < RENDER_WIDTH as f32 {
        frame
            .set(UVec2::new(north as u32, horizon), Color::BLACK)
            .ok();
    }
    let south = project_x(player.direction, std::f32::consts::PI);
    if south > 0.0 && south < RENDER_WIDTH as f32 {
        frame
            .set(UVec2::new(south as u32, horizon), Color::srgb_u8(255, 0, 0))
            .ok();
    }
    let east = project_x(player.direction, std::f32::consts::FRAC_PI_2);
    if east > 0.0 && east < RENDER_WIDTH as f32 {
        frame
            .set(UVec2::new(east as u32, horizon), Color::srgb_u8(0, 255, 0))
            .ok();
    }
    let west = project_x(player.direction, 3.0 * std::f32::consts::FRAC_PI_2);
    if west > 0.0 && west < RENDER_WIDTH as f32 {
        frame
            .set(
                UVec2::new(west as u32, horizon),
                Color::srgb_u8(0, 255, 255),
            )
            .ok();
    }

    //let g = giants.single();
    //render_giant(&mut frame, 10, g.frame == 1);
}

// Translates a value from one Range into the value of another range,
// e.g. 3 (the middle of) in 2..=4 ->  13 in 11..15 (also the middle)
fn lintra(
    value: i32,
    original_range: std::ops::RangeInclusive<i32>,
    target_range: std::ops::RangeInclusive<i32>,
) -> i32 {
    let ratio_in_orig = (value - original_range.start()) as f32
        / (original_range.end() - original_range.start()) as f32;

    ((target_range.end() - target_range.start()) as f32 * ratio_in_orig).round() as i32
}

// Translates a value from one Range into the value of another range,
// e.g. 3 (the middle of) in 2..=4 ->  13 in 11..15 (also the middle)
// TODO how to get "Range<u8,u8>" type?
// takes_range<R: RangeBounds<i32>>(range: R)
// otherwise Range<i32> and take_range(r: RangeInclusive<i32>)
fn linp(start: i32, end: i32, scale_start: i32, scale_end: i32, actual_value: u8) -> u8 {
    //println!(
    //    "{}, {}, {}, {}, {}",
    //    start, end, scale_start, scale_end, actual_value
    //);
    let ratio = (actual_value as i32 - scale_start) as f32 / (scale_end - scale_start) as f32;
    //println!(
    //    "{}, {}, {}, {}, {}",
    //    ratio,
    //    scale_end - scale_start,
    //    actual_value as i32 - scale_start,
    //    end - start,
    //    (end - start) as f32 * ratio
    //);

    if scale_end == actual_value as i32 {
        return end.try_into().unwrap();
    }

    (start as f32 + ((end - start) as f32 * ratio)).round() as u8
}

fn draw_sky(horizon: u8, frame: &mut Frame, bright_up: bool) {
    for y in 0..horizon {
        //let horizon_dist = horizon - y;
        let add = if bright_up {
            linp(50, 0, 0, horizon.try_into().unwrap(), y as u8)
        } else {
            linp(0, 50, 0, horizon.try_into().unwrap(), y as u8)
        };
        //dbg!(add);
        let color = Color::srgba_u8(1 + add, 2 + add, 3 + add, 255);
        for x in 0..RENDER_WIDTH {
            frame.set(UVec2::new(x, y.try_into().unwrap()), color).ok();
        }
    }
}

fn draw_ground(horizon: u32, frame: &mut Frame, bright_up: bool) {
    for y in horizon..RENDER_HEIGHT {
        let add = if bright_up {
            linp(
                45,
                5,
                horizon.try_into().unwrap(),
                RENDER_HEIGHT as i32,
                y as u8,
            )
        } else {
            linp(
                5,
                45,
                horizon.try_into().unwrap(),
                RENDER_HEIGHT as i32,
                y as u8,
            )
        };
        let color = Color::srgba_u8(1 + add, 2 + add, 3 + add, 255);
        for x in 0..RENDER_WIDTH {
            // TODO parameterize
            // scale brightness according to diff from a point
            let pixpos = UVec2::new(x, y);
            let light_dist =
                f32::sqrt(((64 - x as i32) as f32).powf(2.0) + ((50 - y) as f32).powf(2.0));
            let fac = (200.0 - lintra(light_dist as i32, 0..=80, 0..=300) as f32) / 10000.0;
            frame.set(pixpos, color.lighter(fac)).ok();
        }
    }
}

#[allow(non_snake_case)]
fn render_giant(frame: &mut Frame, _xpix: i32, flip: bool) {
    // This needs #![allow(uncommon_codepoints)] to de-warn,
    // lets find another nice "empty looking" identifier.
    // Bitmap of a giant
    const ˑ: bool = true;
    const Ø: bool = false;
    const GIANT_BITMAP: [[bool; 19]; 24] = [
        [Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø],
        [Ø, Ø, Ø, Ø, Ø, ˑ, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø],
        [Ø, Ø, Ø, Ø, Ø, ˑ, ˑ, ˑ, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø],
        [Ø, Ø, Ø, ˑ, ˑ, ˑ, ˑ, ˑ, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø],
        [Ø, Ø, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø],
        [Ø, Ø, ˑ, ˑ, Ø, ˑ, ˑ, ˑ, ˑ, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø],
        [Ø, Ø, Ø, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø],
        [Ø, Ø, Ø, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø],
        [Ø, Ø, Ø, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø],
        [Ø, Ø, Ø, Ø, Ø, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, Ø, Ø, Ø, Ø, Ø],
        [Ø, Ø, Ø, Ø, Ø, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, Ø, Ø, Ø, Ø],
        [Ø, Ø, Ø, Ø, Ø, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, Ø, Ø, Ø],
        [Ø, Ø, Ø, Ø, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, Ø, ˑ, ˑ, ˑ, ˑ, Ø, Ø],
        [Ø, Ø, Ø, Ø, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, Ø, Ø, ˑ, ˑ, ˑ, Ø, Ø],
        [Ø, Ø, Ø, Ø, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, Ø, Ø, Ø, Ø, ˑ, ˑ, Ø],
        [Ø, Ø, Ø, Ø, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, Ø, Ø, Ø, ˑ, ˑ, Ø],
        [Ø, Ø, Ø, Ø, Ø, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, Ø, Ø, Ø, Ø, Ø, Ø],
        [Ø, Ø, Ø, Ø, Ø, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, Ø, Ø, Ø, Ø, Ø, Ø],
        [Ø, Ø, Ø, Ø, Ø, Ø, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, Ø, Ø, Ø, Ø, Ø, Ø],
        [Ø, Ø, Ø, Ø, Ø, Ø, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, Ø, Ø, Ø, Ø, Ø, Ø],
        [Ø, Ø, Ø, Ø, Ø, Ø, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, ˑ, Ø, Ø, Ø, Ø, Ø, Ø],
        [Ø, Ø, Ø, Ø, Ø, Ø, Ø, ˑ, ˑ, ˑ, Ø, ˑ, ˑ, Ø, Ø, Ø, Ø, Ø, Ø],
        [Ø, Ø, Ø, Ø, Ø, Ø, Ø, ˑ, ˑ, Ø, Ø, Ø, ˑ, Ø, Ø, Ø, Ø, Ø, Ø],
        [Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø, Ø],
    ];

    let offset = if flip { 4 } else { 6 };

    // TODO clipping?
    for (y, row) in GIANT_BITMAP.iter().enumerate() {
        for (x, _col) in row.iter().enumerate().filter(|(_, v)| **v) {
            let _ = frame.set(UVec2::new(x as u32 + 30, y as u32 + offset), HORIZON_COL);
        }
    }
}

// radians project to screen
fn project_x(view_direction: f32, obj_dir: f32) -> f32 {
    // OPTIMIZE:
    //   the view_direction - HALF_VIEW_ANGLE is constant per render pass, as is the
    //   RENDER_WIDTH/VIEW_ANGLE
    let k = clockwise_diff(view_direction - HALF_VIEW_ANGLE, obj_dir);
    k / VIEW_ANGLE * (RENDER_WIDTH as f32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq /*assert_ne*/};

    #[test]
    fn linp_interpolates() {
        //assert_eq!(linp(0, 99, 100, 199, 140), 40); :(
        assert_eq!(linp(0, 100, 0, 24, 24), 100);
        assert_eq!(linp(0, 100, 0, 24, 0), 0);
        assert_eq!(linp(2, 180, 0, 24, 7), 40);
    }
}

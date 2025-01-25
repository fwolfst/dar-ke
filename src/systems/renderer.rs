use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;

use crate::components::*;
use crate::systems::render::projector::*;
use crate::{RENDER_HEIGHT, RENDER_WIDTH};

pub const VIEW_ANGLE: f32 = std::f32::consts::PI / 2.0;
pub const HALF_VIEW_ANGLE: f32 = VIEW_ANGLE / 2.0;

const HORIZON_COL: [u8; 3] = [1, 2, 3];

pub fn render(
    mut pb: QueryPixelBuffer,
    player: Query<&Player>,
    giants: Query<(&Giant, &Positioned)>,
    lights: Query<(&Light, &AtHorizon)>,
    blobs: Query<&Blob>,
    pebbles: Query<&Pebble>,
    glitch_blobs: Query<(&GlitchBlob, &Height)>,
    params: Res<Params>,
    sky_blender: Res<SkyBlender>,
) {
    let player = player.single();
    let mut frame = pb.frame();
    let projector = make_projector(player.direction);

    // TODO sky_horizon_ratio now looks like view_angle (up or down)
    // TODO head and height should be proportional to resolution
    let horizon = ((RENDER_HEIGHT as f32 * params.sky_horizon_ratio) as i32
        + player.height as i32
        + player.head as i32)
        .clamp(0, RENDER_HEIGHT as i32) as u32;

    draw_sky(
        &projector,
        horizon as u8,
        &mut frame,
        params.sky_up_bright,
        &sky_blender,
    );

    draw_ground(horizon, &mut frame, params.ground_up_bright, &params);

    for (light, at_horizon) in &lights {
        let pos_of_obj_screen = projector.screen_x_of_rad(at_horizon.angle) as u32;
        frame
            .set([pos_of_obj_screen, horizon - 1], light.color)
            .ok();
    }

    if params.draw_poles {
        draw_poles(&projector, horizon, &mut frame);
    }

    for (b, h) in &glitch_blobs {
        render_glitch_blob(&projector, horizon, &mut frame, &player, b, h);
    }

    for b in &blobs {
        render_blob(&projector, horizon, &mut frame, &player, b);
    }

    for p in &pebbles {
        render_pebble(&projector, horizon, &mut frame, &player, p);
    }

    for (g, p) in &giants {
        render_giant(&projector, &mut frame, 10, &g, &p, &player);
    }
}

fn render_glitch_blob(
    projector: &Projector,
    horizon: u32,
    frame: &mut Frame,
    player: &Player,
    blob: &GlitchBlob,
    height: &Height,
) {
    // Nice, 360 degree view gives nice effects, too :D
    let dx = player.x - blob.x;
    let dy = player.y - blob.y;

    let ab = if dy == 0. {
        0.0
    } else {
        (dx as f32 / dy as f32).atan()
    };

    let bx = projector.screen_x_of_rad(ab);

    frame
        .set([bx as u32, horizon - height.height as u32], [180, 180, 180])
        .ok();
}

fn render_pebble(
    projector: &Projector,
    horizon: u32,
    frame: &mut Frame,
    player: &Player,
    pebble: &Pebble,
) {
    // IDEA size component (visible until, e.g. pebbles: size 10, visible if distance < 10)
    const MAX_PEBBLE_VIEWING_DISTANCE: f32 = 10.0;
    // TODO fix bug with pebbles left of vd
    let dx = player.x - pebble.x;
    let dy = player.y - pebble.y;

    let db = f32::sqrt(dx.powf(2.0) + dy.powf(2.0));
    if !(0.0..MAX_PEBBLE_VIEWING_DISTANCE).contains(&db) {
        return;
    }

    // "North" clockwise
    let ab = if dx == 0. {
        0.0
    } else {
        std::f32::consts::PI + dx.atan2(dy)
    };

    let max_down = (RENDER_HEIGHT - horizon) as f32;

    let k = 1.0; // decy
    let dist = max_down * f32::exp(-k * db);

    let bx = projector.screen_x_of_rad(ab);

    frame.set([bx as u32, horizon + dist.round() as u32], [20, 20, 20]);
}

fn render_blob(
    projector: &Projector,
    horizon: u32,
    frame: &mut Frame,
    player: &Player,
    blob: &Blob,
) {
    // TODO fix bug with pebbles left of vd
    let dx = player.x - blob.x;
    let dy = player.y - blob.y;

    // angle
    //let ab = if dx == 0. { 0.0 } else { dy.atan2(dx) };
    // "North" clockwise
    let ab = if dx == 0. {
        0.0
    } else {
        std::f32::consts::PI + dx.atan2(dy)
    };

    let max_down = (RENDER_HEIGHT - horizon) as f32;

    let db = f32::sqrt(dx.powf(2.0) + dy.powf(2.0));
    let k = 1.0; // decy
    let dist = max_down * f32::exp(-k * db);

    //println!(
    //    "obj angle {} / deg {} / dx {} dy {}",
    //    ab,
    //    ab * 180.0 / std::f32::consts::PI,
    //    dx,
    //    dy
    //);
    let bx = projector.screen_x_of_rad(ab);

    if db < 10.0 {
        // shape
        // else
        // point
    }
    frame.set([bx as u32, horizon + dist.round() as u32], [20, 20, 20]);
}

fn lintra(
    value: i32,
    original_range: std::ops::RangeInclusive<i32>,
    target_range: std::ops::RangeInclusive<i32>,
) -> i32 {
    let ratio_in_orig = (value - original_range.start()) as f32
        / (original_range.end() - original_range.start()) as f32;

    ((target_range.end() - target_range.start()) as f32 * ratio_in_orig).round() as i32
}

fn flintra(
    value: i32,
    original_range: std::ops::RangeInclusive<i32>,
    target_range: std::ops::RangeInclusive<f32>,
) {
    // with clamping?
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

fn draw_sky(
    projector: &Projector,
    horizon: u8,
    frame: &mut Frame,
    bright_up: bool,
    sky_blender: &Res<SkyBlender>,
) {
    for y in 0..horizon {
        let add = if bright_up {
            linp(50, 0, 0, horizon.into(), y)
        } else {
            linp(0, 50, 0, horizon.into(), y)
        };

        let light_xposses = projector.screen_x2_of_rad(std::f32::consts::FRAC_PI_2);

        let light_pos1 = Vec2::new(
            light_xposses.0 as f32,
            // 0 is also nice  RENDER_WIDTH as f32 / 2.0,
            (horizon as i32 - sky_blender.height) as f32,
        );
        let light_pos2 = Vec2::new(
            light_xposses.1 as f32,
            // 0 is also nice  RENDER_WIDTH as f32 / 2.0,
            (horizon as i32 - sky_blender.height) as f32,
        );

        for x in 0..RENDER_WIDTH {
            // Problem here is that we can get double light.
            let light_dist1 = (light_pos1 - Vec2::new(x as f32, y as f32)).length();
            let mut reddener = ((100.0 - light_dist1) / 1000.0).clamp(0.0, 0.2);
            let light_dist2 = (light_pos2 - Vec2::new(x as f32, y as f32)).length();
            reddener += ((100.0 - light_dist2) / 1000.0).clamp(0.0, 0.2);

            let color = Color::srgba_u8(
                1 + add + (reddener * 255.0).round() as u8,
                2 + add,
                3 + add,
                255,
            );
            //color.red;
            frame.set(UVec2::new(x, y.try_into().unwrap()), color).ok();
        }
    }
}

fn draw_poles(projector: &Projector, horizon: u32, frame: &mut Frame) {
    // Draw poles
    // N = black, S = red, E = green, W = yellow
    let north = projector.screen_x_of_rad(0.0);
    dbg!(north);
    if north >= 0 && north < RENDER_WIDTH as i32 {
        frame.set([north as u32, horizon], Color::BLACK).ok();
    }
    let south = projector.screen_x_of_rad(std::f32::consts::PI);
    if south >= 0 && south < RENDER_WIDTH as i32 {
        frame
            .set([south as u32, horizon], Color::srgb_u8(255, 0, 0))
            .ok();
    }
    let east = projector.screen_x_of_rad(std::f32::consts::FRAC_PI_2);
    if east >= 0 && east < RENDER_WIDTH as i32 {
        frame
            .set([east as u32, horizon], Color::srgb_u8(0, 255, 0))
            .ok();
    }
    let west = projector.screen_x_of_rad(3.0 * std::f32::consts::FRAC_PI_2);
    if west > 0 && west < RENDER_WIDTH as i32 {
        frame
            .set([west as u32, horizon], Color::srgb_u8(0, 255, 255))
            .ok();
    }
}

fn draw_ground(horizon: u32, frame: &mut Frame, bright_up: bool, params: &Res<Params>) {
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
        let light_origin_y = RENDER_HEIGHT as i32 + params.light_cone_off_y;
        let light_origin_x = RENDER_WIDTH as i32 / 2 + params.light_cone_off_x;
        for x in 0..RENDER_WIDTH {
            let pixpos = UVec2::new(x, y);
            let light_dist = f32::sqrt(
                ((light_origin_x - x as i32) as f32).powf(2.0)
                    + ((light_origin_y - y as i32) as f32).powf(2.0),
            );

            // Does ignore the decay/distance from params, and function is ... odd.
            let fac = (200.0 - lintra(light_dist as i32, 0..=(80), 0..=300) as f32) / 10000.0;

            frame.set(pixpos, color.lighter(fac)).ok();
        }
    }
}

#[allow(non_snake_case)]
fn render_giant(
    projector: &Projector,
    frame: &mut Frame,
    _xpix: i32,
    giant: &Giant,
    position: &Positioned,
    player: &Player,
) {
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

    let offset = if giant.frame == 1 { 4 } else { 6 };

    let dx = player.x - position.x;
    let dy = player.y - position.y;

    let ab = if dy == 0. {
        0.0
    } else {
        (dx as f32 / dy as f32).atan()
    };

    let sx = projector.screen_x_of_rad(ab);

    // TODO clipping?
    for (y, row) in GIANT_BITMAP.iter().enumerate() {
        for (x, _col) in row.iter().enumerate().filter(|(_, v)| **v) {
            let _ = frame.set(
                UVec2::new(x as u32 + sx as u32, y as u32 + offset),
                HORIZON_COL,
            );
        }
    }
    // TODO only draw till horizon
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

use crate::components::Giant;
use crate::components::Params;
use crate::radians_math::*;
use crate::{components::Player, HALF_VIEW_ANGLE, RENDER_HEIGHT, RENDER_WIDTH, VIEW_ANGLE};
use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;

//use crate::PI_F32;

pub fn render(
    mut pb: QueryPixelBuffer,
    player: Query<&Player>,
    giants: Query<&Giant>,
    params: Res<Params>,
) {
    let mut frame = pb.frame();
    let mut horizon = frame.size().y / 2;

    let player = player.single();
    horizon = ((horizon as i32) + (player.height)) as u32;

    const HORIZON_COL: Pixel = Pixel {
        r: 1,
        g: 2,
        b: 3,
        a: 255,
    };

    let mut red_mult = 0.5 * (1.0 - player.direction);
    let mut green_mult = 0.5 * (-1.0 + player.direction);
    if params.sky_dir_up {
        red_mult = 1.0 / red_mult;
        green_mult = 1.0 / green_mult;
    }

    let left_world = player.direction - HALF_VIEW_ANGLE;
    //let right_world = player.direction + HALF_VIEW_ANGLE;

    let pos_of_obj_world = 0.0;
    let raddiff = rad_wrap_diff(left_world, pos_of_obj_world);
    let pos_of_obj_screen = ((raddiff - left_world) / VIEW_ANGLE) * RENDER_WIDTH as f32;

    // Draw Sky
    for y in 0..horizon {
        //let horizon_dist = horizon - y;
        let add = if params.sky_dir_up {
            linp(0, 12, 0, horizon.try_into().unwrap(), y as u8)
        } else {
            linp(12, 0, 0, horizon.try_into().unwrap(), y as u8)
        };
        dbg!(add);
        let color = Pixel {
            r: 1 + add,
            g: 2 + add,
            b: 3 + add,
            a: 255,
        };
        for x in 0..RENDER_WIDTH {
            frame
                .set(
                    UVec2::new(x, y),
                    color, //Pixel {
                           //    r: (1 + (horizon_dist as f32 * red_mult) as u32).rem_euclid(255) as u8,
                           //    g: 2,
                           //    b: (3 + (horizon_dist as f32 * green_mult) as u32).rem_euclid(255) as u8,
                           //    a: 255,
                           //},
                )
                .ok();
        }
    }

    // Draw Horizon
    for y in horizon..RENDER_HEIGHT {
        let add = if params.ground_dir_up {
            linp(
                0,
                4,
                horizon.try_into().unwrap(),
                RENDER_HEIGHT as i32,
                y as u8,
            )
        } else {
            linp(
                10,
                0,
                horizon.try_into().unwrap(),
                RENDER_HEIGHT as i32,
                y as u8,
            )
        };
        // HORIZON_COL +
        let color = Pixel {
            r: 1 + add,
            g: 2 + add,
            b: 3 + add,
            a: 255,
        };
        for x in 0..RENDER_WIDTH {
            frame.set(UVec2::new(x, y), color).ok();
        }
    }

    // Draw Object
    let _ = frame.set(
        UVec2::new(pos_of_obj_screen as u32, horizon),
        Pixel {
            r: 190,
            g: 190,
            b: 100,
            a: 190,
        },
    );

    let pos_of_obj_world = 1.6;
    let raddiff = rad_wrap_diff(left_world, pos_of_obj_world);
    let pos_of_obj_screen = ((raddiff - left_world) / VIEW_ANGLE) * RENDER_WIDTH as f32;
    // Draw Object
    let _ = frame.set(
        UVec2::new(pos_of_obj_screen as u32, horizon - 1),
        Pixel {
            r: 190,
            g: 190,
            b: 100,
            a: 190,
        },
    );
    let pos_of_obj_world = 0.6;
    let raddiff = rad_wrap_diff(left_world, pos_of_obj_world);
    let pos_of_obj_screen = ((raddiff - left_world) / VIEW_ANGLE) * RENDER_WIDTH as f32;
    // Draw Object
    let _ = frame.set(
        UVec2::new(pos_of_obj_screen as u32, horizon - 1),
        Pixel {
            r: 190,
            g: 190,
            b: 100,
            a: 190,
        },
    );

    let g = giants.single();
    render_giant(&mut frame, 10, g.frame == 1);
}

// TODO how to get "Range<u8,u8>" type?
// takes_range<R: RangeBounds<i32>>(range: R)
// otherwise Range<i32> and take_range(r: RangeInclusive<i32>)
fn linp(start: i32, end: i32, scale_start: i32, scale_end: i32, actual_value: u8) -> u8 {
    println!(
        "{}, {}, {}, {}, {}",
        start, end, scale_start, scale_end, actual_value
    );
    let ratio = (actual_value as i32 - scale_start) as f32 / (scale_end - scale_start) as f32;
    println!(
        "{}, {}, {}, {}, {}",
        ratio,
        scale_end - scale_start,
        actual_value as i32 - scale_start,
        end - start,
        (end - start) as f32 * ratio
    );

    if scale_end == actual_value as i32 {
        return end.try_into().unwrap();
    }

    (start as f32 + ((end - start) as f32 * ratio)).round() as u8
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
            let _ = frame.set(
                UVec2::new(x as u32 + 30, y as u32 + offset),
                Pixel {
                    r: 1,
                    g: 2,
                    b: 3,
                    a: 255,
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linp_interpolates() {
        //assert_eq!(linp(0, 99, 100, 199, 140), 40); :(
        assert_eq!(linp(0, 100, 0, 24, 24), 100);
        assert_eq!(linp(0, 100, 0, 24, 0), 0);
        assert_eq!(linp(2, 180, 0, 24, 7), 40);
    }
}

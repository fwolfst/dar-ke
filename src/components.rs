use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::{radians_math::norm_rad, RENDER_HEIGHT, RENDER_WIDTH};

// Unfortunately no examples and I cannot yet read and understand
// that code :(
//use radians::Rad32;
//use radian::Rad32;

#[derive(Component)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub height: i32,
    // radians between 0 and +2PI
    pub direction: f32,
    pub walking_time: Stopwatch,
    pub is_moving: bool,
}

impl Player {
    // Turn and wrap around (0..+2PI)
    pub fn turn(&mut self, val: f32) {
        self.direction = norm_rad(self.direction + val);
    }
}

#[derive(Component)]
pub struct Giant {
    pub timer: Timer,
    pub frame: u8,
}

#[derive(Clone, Debug, Component)]
pub struct AtHorizon {
    pub angle: f32, // radians
}

#[derive(Clone, Debug, Component)]
pub struct Light {
    pub color: Color,
}

/// These are experiment values that should probably
/// get fixed (and thus baked in) once.
#[derive(Clone, Debug, Resource)]
pub struct Params {
    pub sky_up_bright: bool,
    pub ground_up_bright: bool,
    pub sky_horizon_ratio: f32,
    pub light_cone_off_y: i32,
    pub light_cone_off_x: i32,
    pub light_cone_max_dist: f32,
    pub draw_poles: bool,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            sky_up_bright: false,
            ground_up_bright: false,
            sky_horizon_ratio: 0.6,
            light_cone_off_y: 50 - RENDER_HEIGHT as i32,
            light_cone_off_x: 64 - RENDER_WIDTH as i32 / 2,
            light_cone_max_dist: 12.0,
            draw_poles: false,
        }
    }
}

#[derive(Clone, Debug, Component)]
pub struct Narrative {
    pub timer: Timer,
    //pub phrase_index: usize,
}

impl Default for Narrative {
    fn default() -> Self {
        Narrative {
            timer: Timer::new(Duration::from_secs(8), TimerMode::Once),
            //phrase_index: 0,
        }
    }
}

use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

// Unfortunately no examples and I cannot yet read and understand
// that code :(
//use radians::Rad32;
//use radian::Rad32;

#[derive(Component)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub height: i32,
    // radians between -PI and +PI
    pub direction: f32,
    pub walking_time: Stopwatch,
    pub is_moving: bool,
}

impl Player {
    // Turn and wrap around (-PI..+PI)
    pub fn turn(&mut self, val: f32) {
        self.direction += val;
        if self.direction > std::f32::consts::PI {
            self.direction -= 2.0 * std::f32::consts::PI;
        } else if self.direction < -std::f32::consts::PI {
            self.direction += 2.0 * std::f32::consts::PI;
        }
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

//#[derive(Clone, Debug, Component)]
//pub struct Shape {
//    pub color: Color,
//}

/// These are experiment values that should probably
/// get fixed (and thus baked in) once.
#[derive(Clone, Debug, Resource)]
pub struct Params {
    pub sky_dir_up: bool,
    pub ground_dir_up: bool,
    pub sky_horizon_ratio: f32,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            sky_dir_up: false,
            ground_dir_up: false,
            sky_horizon_ratio: 0.6,
        }
    }
}

#[derive(Clone, Debug, Component)]
pub struct Narrative {
    pub timer: Timer,
    pub phrase_index: usize,
}

impl Default for Narrative {
    fn default() -> Self {
        Narrative {
            timer: Timer::new(Duration::from_secs(8), TimerMode::Once),
            phrase_index: 0,
        }
    }
}

//impl Default for Player {
//    fn default() -> Self {
//        Self {
//            y: 128,
//            x: 1.0,
//            height: Vec2::ZERO,
//        }
//    }
//}

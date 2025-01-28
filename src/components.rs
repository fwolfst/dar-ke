use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::systems::renderer::HORIZON_WIDTH_IN_PIXEL;
use crate::{radians_math::norm_rad, RENDER_HEIGHT, RENDER_WIDTH};

// Unfortunately no examples and I cannot yet read and understand
// that code :(
//use radians::Rad32;
//use radian::Rad32;

#[derive(Component)]
pub struct Player {
    pub x: f32,         // pos x
    pub y: f32,         // pos y
    pub height: i32,    // Base height
    pub head: i32,      // Head bobbing
    pub direction: f32, // rad 0..+2PI
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

#[derive(Component)]
pub struct Positioned {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct IsFalling;

#[derive(Component)]
pub struct Height {
    pub height: f32,
}

#[derive(Clone, Debug, Component)]
pub struct AtHorizon {
    pub angle: f32, // radians
}

#[derive(Clone, Debug, Component)]
pub struct Light {
    pub color: Color,
}

#[derive(Clone, Debug, Component)]
pub struct Pebble {
    pub x: f32,
    pub y: f32,
    //pub color: Color,
}

#[derive(Component)]
pub struct Blob {
    pub color: Color,
    //pub position: Vec2,
    pub x: f32,
    pub y: f32,
    //pub height: i32,
}

#[derive(Clone, Debug, Resource)]
pub struct SkyBlender {
    pub height: i32,
    //pub position: f32,
    pub extend: f32,
    //pub color: Color,
    //pub strength: f32, // can be alpha of color?
}
impl Default for SkyBlender {
    fn default() -> Self {
        Self {
            height: 5,
            //position: 0.0,
            extend: 30.0,
            //color: Color::srgba_u8(160, 80, 80, 80),
        }
    }
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
    //pub timer: Timer,
    //pub phrase_index: usize,
}

impl Default for Narrative {
    fn default() -> Self {
        Narrative {
            //timer: Timer::new(Duration::from_secs(8), TimerMode::Once),
            //phrase_index: 0,
        }
    }
}

#[derive(Component)]
pub struct GlitchBlob {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource)]
pub struct HorizonBitmap {
    pub data: [u8; (HORIZON_WIDTH_IN_PIXEL) as usize],
}

#[derive(Component)]
pub struct CameraShake {
    pub strength: f32, // between 0 and 1.0
    pub duration: Timer,
}

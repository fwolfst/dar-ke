use std::ops::RangeInclusive;

use bevy::{prelude::*, time::Stopwatch};
use rand::{thread_rng, Rng};

use crate::systems::renderer::HORIZON_WIDTH_IN_PIXEL;
use crate::{radians_math::norm_rad, RENDER_HEIGHT, RENDER_WIDTH};

// Unfortunately no examples and I cannot yet read and understand
// that code :(
//use radians::Rad32;
//use radian::Rad32;

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
    /// Turn and wrap around (0..+2PI)
    pub fn turn(&mut self, val: f32) {
        self.direction = norm_rad(self.direction + val);
    }

    /// Create a new Position vector
    pub fn pos_as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
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
}

#[derive(Component)]
pub struct Blob {
    pub color: Color,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, Resource)]
pub struct SkyBlender {
    pub height: i32,
    pub extend: f32,
}

impl Default for SkyBlender {
    fn default() -> Self {
        Self {
            height: 5,
            extend: 30.0,
        }
    }
}

#[derive(Clone, Debug, Component)]
pub struct Narrative;

#[derive(Component)]
pub struct GlitchBlob {
    pub x: f32,
    pub y: f32,
    pub color: Color,
}

#[derive(Component)]
pub struct Fading {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct HorizonBitmap {
    pub data: [u8; (HORIZON_WIDTH_IN_PIXEL) as usize],
}

impl HorizonBitmap {
    /// Generates a horizon made of simple, symetric triangles.
    /// The triangles can actually double as wide as given.
    pub fn generate(
        num_objs: i32,
        width: RangeInclusive<u32>,
        height: RangeInclusive<u32>,
    ) -> Self {
        let mut rng = thread_rng();
        let mut data = [0; HORIZON_WIDTH_IN_PIXEL as usize];

        for _ in 0..num_objs {
            let pos = rng.gen_range(0..(HORIZON_WIDTH_IN_PIXEL as u32));
            let width = rng.gen_range(width.clone());
            let peak = rng.gen_range(height.clone());

            for x in 0..width {
                let height = ((x as f32 / width as f32) * peak as f32).round() as u8; // triangle
                let idx = ((pos + x) % HORIZON_WIDTH_IN_PIXEL as u32) as usize;
                data[idx] = height.max(data[idx]);
                let idx = ((pos + 2 * width - x - 1) % HORIZON_WIDTH_IN_PIXEL as u32) as usize;
                data[idx] = height.max(data[idx]);
            }
        }

        Self { data }
    }
}

#[derive(Component)]
pub struct CameraShake {
    pub strength: f32, // between 0 and 1.0
    pub duration: Timer,
}

#[derive(Component)]
pub struct CreditRoll {
    pub time: Timer, // between 0 and 1.0
}

#[derive(Component, Reflect)]
pub struct Colored(pub Color);

#[derive(Component)]
pub struct Fly;

pub struct Bird;

#[derive(Component)]
pub struct Tree;

#[derive(Component)]
pub struct Stage1Blob;

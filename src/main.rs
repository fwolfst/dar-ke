// For bitmap (non-)ASCII art in src/systems/renderer.rs
#![allow(uncommon_codepoints)]

use std::time::Duration;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    time::Stopwatch,
};
use bevy_egui::{
    egui::{self, RichText},
    EguiContexts, EguiPlugin,
};

use bevy_pixel_buffer::prelude::*;
use rand::{thread_rng, Rng};

use components::Narrative;
use components::{Giant, SkyBlender}; // not needed in src/main, but reincluded through it -> need to learn
                                     // and think of how to structure imports
                                     // Cannot yet read the code good enough to figure out
                                     // how to use these (and they lack examples :( )
                                     //use radians::Rad32;
                                     //use radian::Rad32;

mod components;
mod phrases;
pub mod radians_math;
mod systems;

use crate::components::Blob;
use crate::components::GlitchBlob;
use crate::components::Height;
use crate::components::HorizonBitmap;
use crate::components::Params;
use crate::components::Pebble;
use crate::components::Player;
use crate::systems::input::*;
use crate::systems::physics::*;
use crate::systems::renderer::*;
use crate::systems::run_intro::*;
use crate::systems::ui::*;
use crate::systems::world::*;

// TODO It makes sense to play with these constants
// and unconstantize them into the Params compoment
// until settled.
pub const RENDER_WIDTH: u32 = 128;
pub const RENDER_HEIGHT: u32 = 48;
pub const PIXEL_SIZE: u32 = 13;
use crate::systems::renderer::HORIZON_WIDTH_IN_PIXEL;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Intro,
    Playing,
    Credits,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                //mode: bevy::window::WindowMode::BorderlessFullscreen,
                resolution: bevy::window::WindowResolution::default(),
                //cursor: Cursor {
                //    visible: false,
                //    ..default()
                //},
                ..default()
            }),
            ..default()
        }),
        EguiPlugin,
        FrameTimeDiagnosticsPlugin,
        PixelBufferPlugin,
    ));
    app.add_systems(
        Startup,
        (
            init_pixel_buffer,
            init_player,
            init_pebble_field,
            init_stage1,
            spawn_darke,
        ),
    )
    .add_systems(
        FixedUpdate,
        (process_input, area_effects).run_if(in_state(GameState::Playing)),
    )
    .add_systems(
        Update,
        (
            run_intro.run_if(in_state(GameState::Intro)),
            ui,
            physics,
            animate,
            update.after(physics),
            render.after(update),
        ),
    )
    .insert_state(GameState::Intro)
    .insert_resource(Params::default())
    .insert_resource(SkyBlender::default())
    .insert_resource(generate_horizon())
    .run();
}

fn init_player(mut commands: Commands) {
    commands.spawn(Player {
        x: 0.,     // will move during intro
        y: -2000., // will move during intro
        head: 0,
        height: 3,
        direction: 0.0,
        walking_time: Stopwatch::new(),
        is_moving: false,
    });
}

fn init_pixel_buffer(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    PixelBufferBuilder::new()
        .with_size(PixelBufferSize {
            size: UVec2::new(RENDER_WIDTH, RENDER_HEIGHT),
            pixel_size: UVec2::new(PIXEL_SIZE, PIXEL_SIZE),
        })
        .with_fill(Fill::none().with_stretch(false))
        .spawn(&mut commands, &mut images)
        .entity();
}

fn init_pebble_field(mut commands: Commands) {
    let mut rng = thread_rng();
    for _ in 0..10000 {
        commands.spawn(Pebble {
            x: rng.gen_range(-100.0..100.0) as f32,
            y: rng.gen_range(-100.0..100.0) as f32,
        });
    }
}

fn init_stage1(mut commands: Commands) {
    commands.spawn(Blob {
        x: 10.0,
        y: 10.0,
        //height: 2,
        color: Color::srgb_u8(180, 60, 50),
    });
}

fn init_blobs(mut commands: Commands) {
    commands.spawn(Blob {
        x: 0.0,
        y: 20.0,
        color: Color::srgb_u8(130, 160, 150),
        //height: 0,
    });
}

/// generate triangles on the horizon
// TODO more randomness, not always mirrored triangles
fn generate_horizon() -> HorizonBitmap {
    let max_height = 11;
    let min_width = 4; // actually double as wide
    let max_width = 38; // actually double as wide
    let num_objs = 40;

    let mut rng = thread_rng();
    let mut data = [0; HORIZON_WIDTH_IN_PIXEL as usize];

    for _ in 0..num_objs {
        let pos = rng.gen_range(0..(HORIZON_WIDTH_IN_PIXEL as u32));
        let width = rng.gen_range(min_width..max_width);
        let peak = rng.gen_range(1..max_height);

        for x in 0..width {
            let height = ((x as f32 / width as f32) * peak as f32).round() as u8; // triangle
            let idx = ((pos + x) % HORIZON_WIDTH_IN_PIXEL as u32) as usize;
            data[idx] = height.max(data[idx]);
            let idx = ((pos + 2 * width - x - 1) % HORIZON_WIDTH_IN_PIXEL as u32) as usize;
            data[idx] = height.max(data[idx]);
        }
    }
    HorizonBitmap { data }
}

#[allow(non_snake_case)]
fn spawn_darke(mut commands: Commands) {
    const ˑ: bool = true;
    const Ø: bool = false;
    const DARKE: [[bool; 23]; 5] = [
        [
            Ø, Ø, ˑ, ˑ, ˑ, ˑ, Ø, ˑ, ˑ, ˑ, Ø, Ø, Ø, ˑ, ˑ, Ø, ˑ, Ø, ˑ, ˑ, Ø, Ø, Ø,
        ],
        [
            Ø, ˑ, Ø, ˑ, ˑ, Ø, ˑ, Ø, ˑ, ˑ, Ø, ˑ, Ø, ˑ, ˑ, Ø, ˑ, Ø, ˑ, ˑ, Ø, ˑ, ˑ,
        ],
        [
            Ø, ˑ, Ø, ˑ, ˑ, Ø, Ø, Ø, ˑ, ˑ, Ø, Ø, ˑ, ˑ, ˑ, Ø, Ø, ˑ, ˑ, ˑ, Ø, Ø, Ø,
        ],
        [
            Ø, ˑ, Ø, ˑ, ˑ, Ø, ˑ, Ø, ˑ, ˑ, Ø, ˑ, Ø, ˑ, ˑ, Ø, ˑ, Ø, ˑ, ˑ, Ø, ˑ, ˑ,
        ],
        [
            Ø, Ø, ˑ, ˑ, ˑ, Ø, ˑ, Ø, ˑ, ˑ, Ø, ˑ, Ø, ˑ, ˑ, Ø, ˑ, Ø, ˑ, ˑ, Ø, Ø, Ø,
        ],
    ];
    for (y, row) in DARKE.iter().enumerate() {
        for (x, i) in row.iter().enumerate() {
            if !*i {
                commands.spawn((
                    GlitchBlob {
                        x: x as f32 - 10.0,
                        y: 80.0,
                        //color: Color::srgb_u8(160,170,160),
                    },
                    Height {
                        height: 30.0 - y as f32,
                    },
                ));
            }
        }
    }
}


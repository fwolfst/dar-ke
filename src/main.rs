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
use crate::components::Params;
use crate::components::Pebble;
use crate::components::Player;
use crate::systems::input::*;
use crate::systems::physics::*;
use crate::systems::renderer::*;
use crate::systems::ui::*;
use crate::systems::world::*;

// TODO It makes sense to play with these constants
// and unconstantize them into the Params compoment
// until settled.
pub const RENDER_WIDTH: u32 = 128;
pub const RENDER_HEIGHT: u32 = 48;
pub const PIXEL_SIZE: u32 = 13;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    //mode: bevy::window::WindowMode::BorderlessFullscreen,
                    resolution: bevy::window::WindowResolution::default(),
                    ..default()
                }),
                ..default()
            }),
            EguiPlugin,
            FrameTimeDiagnosticsPlugin,
            PixelBufferPlugin,
        ))
        .add_systems(Startup, (init_pixel_buffer, init_player, init_stage1))
        .add_systems(FixedUpdate, (process_input, area_effects))
        .add_systems(
            Update,
            (
                ui,
                physics,
                animate,
                update.after(physics),
                render.after(update),
            ),
        )
        .insert_resource(Params::default())
        .insert_resource(SkyBlender::default())
        .run();
}

fn init_player(mut commands: Commands) {
    commands.spawn(Player {
        x: 0.,
        y: 0.,
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

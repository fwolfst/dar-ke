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
use narration::*;
use rand::{thread_rng, Rng}; // TODO should have one, with known seed.

use components::{Colored, Fading, Fly, Narrative, Positioned, Stage1Blob};
use components::{Giant, SkyBlender};
use stages::*;
use systems::{camera_shake::*, run_credits::run_credits};
// not needed in src/main, but reincluded through it -> need to learn
// and think of how to structure imports
// Cannot yet read the code good enough to figure out
// how to use these (and they lack examples :( )
//use radians::Rad32;
//use radian::Rad32;

mod bitmaps;
mod components;
mod narration;
mod phrases;
pub mod radians_math;
mod stages;
mod systems;

use crate::components::Blob;
use crate::components::GlitchBlob;
use crate::components::Height;
use crate::components::HorizonBitmap;
use crate::components::Params;
use crate::components::Pebble;
use crate::components::Player;
use crate::systems::input::*;
use crate::systems::narrative_fading::*;
use crate::systems::physics::*;
use crate::systems::renderer::*;
use crate::systems::run_intro::*;
use crate::systems::ui::*;
use crate::systems::world::*;

// TODO It makes sense to play with these constants
// and unconstantize them into the Params compoment
// until settled.
use crate::systems::renderer::RENDER_HEIGHT;
use crate::systems::renderer::RENDER_WIDTH;

pub const PIXEL_SIZE: u32 = 13;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Intro,
    Tutorial,
    Playing,
    // Stage1
    // Stage2
    // Decision
    // Stage3
    Credits,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: bevy::window::WindowResolution::default(),
                // For release
                //mode: bevy::window::WindowMode::BorderlessFullscreen,
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
            max_window,
            init_pixel_buffer,
            init_player,
            init_pebble_field,
            spawn_darke,
        ),
    )
    .add_systems(
        FixedUpdate,
        (
            (process_input, area_effects).run_if(in_state(GameState::Playing)),
            (tutorial::input).run_if(in_state(GameState::Tutorial)),
        ),
    )
    .add_systems(
        Update,
        (
            run_intro.run_if(in_state(GameState::Intro)),
            run_credits.run_if(in_state(GameState::Credits)),
            end_it.run_if(in_state(GameState::Playing)),
            ui,
            physics,
            narrative_fading,
            animate_giants,
            animate_birds,
            camera_shake,
            head_bobb.run_if(in_state(GameState::Playing)),
            update.after(physics),
            render.after(update),
        ),
    )
    .add_systems(OnEnter(GameState::Credits), spawn_darke_sky)
    .add_systems(OnEnter(GameState::Tutorial), tutorial::init)
    .add_systems(OnEnter(GameState::Playing), init_stage1)
    .insert_state(GameState::Intro)
    .insert_resource(Params::default())
    .insert_resource(SkyBlender::default())
    .insert_resource(HorizonBitmap::generate(40, 4..=38, 1..=11))
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
    commands.spawn((
        Blob {
            x: 10.0,
            y: 10.0,
            color: Color::srgb_u8(180, 60, 50),
        },
        Stage1Blob,
    ));
}

#[allow(non_snake_case)]
fn spawn_darke(mut commands: Commands) {
    for (y, row) in bitmaps::darke::DARKE.iter().enumerate() {
        for (x, i) in row.iter().enumerate() {
            if !*i {
                commands.spawn((
                    GlitchBlob {
                        x: x as f32 - 10.0,
                        y: 80.0,
                        color: Color::srgb_u8(240, 240, 240),
                    },
                    Height {
                        height: 30.0 - y as f32,
                    },
                ));
            }
        }
    }
}

fn spawn_darke_sky(mut commands: Commands, player: Query<&Player>) {
    let player = player.single();
    let dist = 20.0;
    for (y, row) in bitmaps::darke::DARKE.iter().enumerate() {
        for (x, i) in row.iter().enumerate() {
            if !*i {
                let a = player.direction - 0.5 + x as f32 * 0.01;
                commands.spawn((
                    Fly,
                    Positioned {
                        x: player.x + a.sin() * dist,
                        y: player.y + a.cos() * dist,
                        //color:
                    },
                    Colored(Color::srgb_u8(240, 160, 160)),
                    Height {
                        height: 30.0 - y as f32,
                    },
                ));
            }
        }
    }
}

fn max_window(mut _windows: Query<&mut Window>) {
    // For release
    //let mut window = windows.single_mut();
    //window.set_maximized(true);
}

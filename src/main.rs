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
use components::Giant;
use components::Narrative; // not needed in src/main, but reincluded through it -> need to learn
                           // and think of how to structure imports
                           // Cannot yet read the code good enough to figure out
                           // how to use these (and they lack examples :( )
                           //use radians::Rad32;
                           //use radian::Rad32;

mod components;
mod phrases;
pub mod radians_math;
mod systems;

use crate::components::Params;
use crate::components::Player;
use crate::systems::input::*;
use crate::systems::renderer::*;
use crate::systems::ui::*;
use crate::systems::world::*;

// It makes sense to play with these constants
// and unconstantize them into the Params compoment
// until settled.
pub const RENDER_WIDTH: u32 = 128;
pub const RENDER_HEIGHT: u32 = 48;
pub const PIXEL_SIZE: u32 = 16;
pub const VIEW_ANGLE: f32 = std::f32::consts::PI / 4.0;
pub const HALF_VIEW_ANGLE: f32 = VIEW_ANGLE / 2.0;

pub const PI_F32: f32 = std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: bevy::window::WindowMode::BorderlessFullscreen,
                    resolution: bevy::window::WindowResolution::default(),
                    ..default()
                }),
                ..default()
            }),
            EguiPlugin,
            FrameTimeDiagnosticsPlugin,
            PixelBufferPlugin,
        ))
        .add_systems(Startup, (init_pixel_buffer, init_player, init_giants))
        .add_systems(FixedUpdate, process_input)
        .add_systems(Update, (ui, update, render.after(update)))
        .insert_resource(Params::default())
        .run();
}

fn init_player(mut commands: Commands) {
    commands.spawn(Player {
        x: 0.,
        y: 0.,
        height: 0,
        direction: 0.0,
        walking_time: Stopwatch::new(),
        is_moving: false,
    });
    //for (text_anchor, color) in [
    //    (Anchor::TopLeft, Color::srgb(0.5, 0.5, 0.5)),
    //    (Anchor::TopRight, Color::srgb(0.0, 1.0, 0.5)),
    //    (Anchor::BottomRight, Color::srgb(0.0, 0.0, 1.0)),
    //    (Anchor::BottomLeft, Color::srgb(1., 1., 1.)),
    //] {
    //    commands.spawn(Text2dBundle {
    //        text: Text {
    //            sections: vec![TextSection::new(
    //                format!(" Anchor::{text_anchor:?} "),
    //                TextStyle { color, ..default() },
    //            )],
    //            ..Default::default()
    //        },
    //        //transform: Transform::from_translation(350. * Vec3::Y),
    //        transform: Transform::from_translation(Vec3::new(120.0, 410.0, 0.0)),
    //        //text_anchor,
    //        ..default()
    //    });
    //}
}

fn init_giants(mut commands: Commands) {
    commands.spawn(Giant {
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
        frame: 1,
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

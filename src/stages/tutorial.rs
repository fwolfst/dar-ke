use std::time::Duration;

use bevy::app::AppExit;
use bevy::color::Color;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{
    default, Commands, Entity, EventWriter, KeyCode, NextState, Query, Res, ResMut, With,
};
use bevy::time::{Timer, TimerMode};

use crate::components::{self, Fading, GlitchBlob, Narrative, Player};
use crate::{narration::*, GameState};

pub fn init(mut commands: Commands, darkes: Query<Entity, With<GlitchBlob>>) {
    for e in darkes.iter() {
        commands.entity(e).insert(Fading {
            timer: Timer::new(Duration::from_secs(4), TimerMode::Once),
        });
    }

    let color = Color::srgb_u8(150, 130, 110);
    // Once the key has been pressed, fading component will be added to the letter
    spawn_narrative(
        &mut commands,
        ShowNarrative {
            text: "S".into(),
            color,
            ..default()
        },
    );
    spawn_narrative(
        &mut commands,
        ShowNarrative {
            text: "A".into(),
            position_from_center: Vec2::new(15.0, 0.0),
            ..default()
        },
    );
    spawn_narrative(
        &mut commands,
        ShowNarrative {
            text: "D".into(),
            position_from_center: Vec2::new(30.0, 0.0),
            ..default()
        },
    );
}

/// During intro/turorial: Only one movement possible at a time,
/// fading away the narrative, while spawing others.
pub fn input(
    mut player: Query<&mut Player>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    narratives: Query<(Entity, &components::Name, &Narrative, Option<&Fading>)>,
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    const MOVE_SPEED: f32 = 0.05;
    const TURN_SPEED: f32 = 0.01;

    let s_done = !narration_present("S".into(), &narratives);
    let a_done = !narration_present("A".into(), &narratives);
    let d_done = !narration_present("D".into(), &narratives);
    let w_done = !narration_present("W".into(), &narratives);

    if keyboard_input.pressed(KeyCode::KeyS) {
        // TODO decide if we handle S even if already done

        let mut player = player.single_mut();
        let dx = f32::sin(player.direction);
        let dy = f32::cos(player.direction);
        player.x -= dx * MOVE_SPEED;
        player.y -= dy * MOVE_SPEED;

        // optional headbobbing comes in later
        //player.height = (player.x as i32).rem_euclid(3) - 1;

        fade("S".into(), narratives, &mut commands);

        // TODO only spawn this once.
        spawn_narrative(
            &mut commands,
            ShowNarrative {
                text: "There was no way back".into(),
                position_from_center: Vec2::new(45.0, 45.0),
                ..default()
            },
        );
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        // 'S' not yet done
        if narration_present("S".into(), &narratives) {
            return;
        }

        let mut player = player.single_mut();
        player.turn(-TURN_SPEED);

        fade("A".into(), narratives, &mut commands);
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        // 'A' not yet done
        if narratives.iter().any(|(_e, name, _n, _f)| name.0 == "A") {
            return;
        }

        let mut player = player.single_mut();
        player.turn(TURN_SPEED);

        fade("D".into(), narratives, &mut commands);

        // TODO or only spawn when D is gone?
    } else if keyboard_input.pressed(KeyCode::KeyW) {
        // 'D' not yet done
        if narratives.iter().any(|(_e, name, _n, _f)| name.0 == "D") {
            return;
        }

        let mut player = player.single_mut();
        let dx = f32::sin(player.direction);
        let dy = f32::cos(player.direction);
        player.x += dx * MOVE_SPEED;
        player.y += dy * MOVE_SPEED;

        // Head bobb?

        fade("W".into(), narratives, &mut commands);

        // TODO or only spawn when D is gone?
    } else if keyboard_input.pressed(KeyCode::KeyQ) {
        exit.send(AppExit::Success);
    }

    if s_done && a_done && d_done {
        if w_done {
            // TODO Afterwards, done. Switch to next state and add narrative (for full input handling)
            next_state.set(GameState::Playing);
        } else {
            spawn_narrative(
                &mut commands,
                ShowNarrative {
                    text: "W".into(),
                    position_from_center: Vec2::new(45.0, 0.0),
                    ..default()
                },
            );
        }
    }
}

pub fn narration_present(
    letter: String,
    narratives: &Query<(Entity, &components::Name, &Narrative, Option<&Fading>)>,
) -> bool {
    narratives.iter().any(|(_e, name, _n, _f)| name.0 == letter)
}

pub fn fade(
    letter: String,
    narratives: Query<(Entity, &components::Name, &Narrative, Option<&Fading>)>,
    commands: &mut Commands,
) {
    narratives
        .iter()
        .filter(|(_e, name, _n, f)| name.0 == letter && f.is_none())
        .for_each(|(e, _name, _n, _f)| {
            commands.entity(e).insert(Fading {
                timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Once),
            });
        });
}

pub fn end(mut _commands: Commands) {
    // These are not the droids.
}

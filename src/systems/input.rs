use crate::components::CameraShake;
use crate::*;
use bevy::window::WindowMode;
use bevy::window::*;

/// During intro/turorial: Only one movement possible at a time,
/// fading away the narrative, while spawing others.
pub fn intro_process_input(
    mut player: Query<&mut Player>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    narratives: Query<(Entity, &components::Name, &Narrative, Option<&Fading>)>,
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
) {
    const MOVE_SPEED: f32 = 0.05;
    const TURN_SPEED: f32 = 0.01;

    if keyboard_input.pressed(KeyCode::KeyS) {
        // TODO add narrative: no way back
        // TODO decide if we handle S even if already done

        let mut player = player.single_mut();
        let dx = f32::sin(player.direction);
        let dy = f32::cos(player.direction);
        player.x -= dx * MOVE_SPEED;
        player.y -= dy * MOVE_SPEED;

        //player.height = (player.x as i32).rem_euclid(3) - 1;

        fade("S".into(), narratives, commands);
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        // 'S' not yet done
        if narration_present("S".into(), &narratives) {
            return;
        }

        let mut player = player.single_mut();
        player.turn(-TURN_SPEED);

        fade("A".into(), narratives, commands);
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        // 'A' not yet done
        if narratives.iter().any(|(_e, name, _n, _f)| name.0 == "A") {
            return;
        }

        let mut player = player.single_mut();
        player.turn(TURN_SPEED);

        fade("D".into(), narratives, commands);
    } else if keyboard_input.pressed(KeyCode::KeyQ) {
        exit.send(AppExit::Success);
    }

    // TODO Afterwards, done. Switch to next state and add narrative (for full input handling)
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
    mut commands: Commands,
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

pub fn process_input(
    mut player: Query<&mut Player>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    mut commands: Commands,
    mut windows: Query<&mut Window>,
) {
    const MOVE_SPEED: f32 = 0.1; //1.0;
    const TURN_SPEED: f32 = 0.02;

    if keyboard_input.pressed(KeyCode::KeyA) {
        let mut player = player.single_mut();
        player.turn(-TURN_SPEED);
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        let mut player = player.single_mut();
        player.turn(TURN_SPEED);
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        let mut player = player.single_mut();
        let dx = f32::sin(player.direction);
        let dy = f32::cos(player.direction);
        player.x += dx * MOVE_SPEED;
        player.y += dy * MOVE_SPEED;
        //player.height = (player.x as i32).rem_euclid(3) - 1;
        if !player.is_moving {
            player.is_moving = true;
            player.walking_time.reset();
        }
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        let mut player = player.single_mut();
        let dx = f32::sin(player.direction);
        let dy = f32::cos(player.direction);
        player.x -= dx * MOVE_SPEED;
        player.y -= dy * MOVE_SPEED;
        //player.height = (player.x as i32).rem_euclid(3) - 1;
        if !player.is_moving {
            player.is_moving = true;
            player.walking_time.reset();
        }
    } else {
        let mut player = player.single_mut();
        player.is_moving = false;
        player.head = 0;
    }

    if keyboard_input.pressed(KeyCode::KeyN) {
        commands.spawn(CameraShake {
            duration: Timer::new(Duration::from_secs(2), TimerMode::Once),
            strength: 0.2,
        });
    }

    if keyboard_input.pressed(KeyCode::KeyQ) {
        exit.send(AppExit::Success);
    }
    if keyboard_input.pressed(KeyCode::KeyT) {
        let color = Color::srgb_u8(150, 120, 130);
        commands.spawn((
            components::Narrative,
            Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        phrases::PHRASES[0],
                        TextStyle { color, ..default() },
                    )],
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(120.0, 410.0, 0.0)),
                ..default()
            },
        ));
    }
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        let mut window = windows.single_mut();
        if window.mode != WindowMode::BorderlessFullscreen {
            window.cursor.visible = false;
            window.mode = WindowMode::BorderlessFullscreen
        } else {
            window.cursor.visible = true;
            window.mode = WindowMode::Windowed;
        }
    }
}

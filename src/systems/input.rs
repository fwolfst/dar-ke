use crate::components::CameraShake;
use crate::*;
use bevy::window::WindowMode;
use bevy::window::*;

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

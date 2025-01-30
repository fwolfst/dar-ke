use components::AtHorizon;
use components::CameraShake;
use components::CreditRoll;
use components::Fading;
use components::GlitchBlob;

use crate::*;

use std::f32::consts::PI;

/// Animate the giant(s) walking, including the screen shake
pub fn animate(
    mut giants: Query<(&mut Giant, &mut AtHorizon)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut giant, mut pos) in &mut giants {
        giant.timer.tick(time.delta());
        if giant.timer.finished() {
            giant.frame = (giant.frame + 1).rem_euclid(2);
            if giant.frame == 0 {
                commands.spawn(CameraShake {
                    strength: 1.0,
                    duration: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Once),
                });
            }
        }
        pos.angle -= 0.015 * time.delta_seconds();
    }
}

/// Head bobbin'
pub fn head_bobb(mut player: Query<&mut Player>, time: Res<Time>) {
    // Head bobble
    // sinus curve over 1 second, 1 second transition to baseline if not moving
    for mut player in &mut player {
        player.walking_time.tick(time.delta());
        if player.is_moving {
            player.head =
                (3.0 * (4.0 * player.walking_time.elapsed_secs() % (PI + 0.3)).sin()) as i32;
        }
    }
}

/// Other world updates
pub fn update(
    mut _player: Query<&mut Player>,
    mut fading_blobs: Query<(Entity, &mut GlitchBlob, &mut Fading)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    // tbd
    fading_blobs.iter_mut().for_each(|(e, mut gb, mut f)| {
        f.timer.tick(time.delta());
        gb.color = gb.color.darker(0.006);

        if f.timer.just_finished() {
            commands.entity(e).despawn();
        }
    });
}

/// Bow if leaving the area
pub fn area_effects(_commands: Commands, time: Res<Time>, mut player: Query<&mut Player>) {
    // if outside bounds, bow
    // When leaving/entering the area, give a one-time effect (screen shake)
    let mut player = player.single_mut();
    //if !(-100.0..100.0).contains(&player.x) || !(-100.0..100.0).contains(&player.y) {
    if !(-10.0..10.0).contains(&player.x) || !(-10.0..10.0).contains(&player.y) {
        player.height = 10 - (time.elapsed_seconds() * 3.0) as i32;
    }
}

/// Check if its the end for the player (transition to credits state)
/// Could trigger a one-off system in bevy 15
pub fn end_it(
    mut player: Query<&mut Player>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    let player = player.single_mut();

    if player.height < -10 {
        //params.light_cone_off_y = 90.0;
        next_state.set(GameState::Credits);

        // Remove other Narratives?
        commands.spawn(CameraShake {
            duration: Timer::new(Duration::from_secs_f32(0.2), TimerMode::Once),
            strength: 0.2,
        });

        commands.spawn((
            components::Narrative,
            Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Q(uit)",
                        TextStyle {
                            color: Color::srgb_u8(180, 180, 180),
                            ..default()
                        },
                    )],
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 330.0, 0.0)),
                ..default()
            },
            Fading {
                timer: Timer::new(Duration::from_secs(4), TimerMode::Once),
            },
        ));

        commands.spawn(CreditRoll {
            time: Timer::new(Duration::from_secs(10), TimerMode::Once),
        });
    }
}

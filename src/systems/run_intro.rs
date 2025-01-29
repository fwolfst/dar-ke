use crate::*;

use components::Fading;
use components::Player;

pub fn run_intro(
    mut player: Query<&mut Player>,
    mut darkes: Query<&mut Height, With<GlitchBlob>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    mut params: ResMut<Params>,
    mut commands: Commands,
) {
    // animation will take 6 seconds or so
    let mut player = player.single_mut();

    if player.y <= 0.0 {
        player.y += time.delta_seconds() * 500.0;

        for mut height in darkes.iter_mut() {
            height.height -= 5.0 * time.delta_seconds();
        }

        params.light_cone_off_y =
            (90.0 + player.y * 150.0 / -2000.0).round() as i32 - RENDER_HEIGHT as i32;
    } else {
        // Alternatively: Move these to StateEnter
        player.y = 0.0;
        next_state.set(GameState::Playing);
        let color = Color::srgb_u8(150, 130, 110);
        commands.spawn((
            components::Narrative,
            Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "S A D W",
                        TextStyle { color, ..default() },
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
    }
}

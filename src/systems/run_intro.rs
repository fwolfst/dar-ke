use crate::*;
use components::Player;

pub fn run_intro(
    mut player: Query<&mut Player>,
    mut darkes: Query<&mut Height, With<GlitchBlob>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut player = player.single_mut();
    if player.y <= 0.0 {
        player.y += time.delta_seconds() * 500.0;
    } else {
        next_state.set(GameState::Playing);
    }
    for mut height in darkes.iter_mut() {
        height.height -= 5.0 * time.delta_seconds();
    }
}

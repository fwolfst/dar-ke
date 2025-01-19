use crate::*;

///  tick tickers, animate or despawn.
pub fn update(
    mut giants: Query<&mut Giant>,
    mut player: Query<&mut Player>,
    mut narrative: Query<(Entity, &mut Narrative)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    // animate giant wobble
    for mut giant in &mut giants {
        giant.timer.tick(time.delta());
        if giant.timer.finished() {
            giant.frame = (giant.frame + 1).rem_euclid(2);
        }
    }

    // Fade out narrative
    for (entity, mut narrative) in &mut narrative {
        narrative.timer.tick(time.delta());
        if narrative.timer.finished() {
            commands.entity(entity).despawn();
        }
    }

    // Head wobble
    // sinus curve over 1 second, 1 second transition to baseline if not moving
    for mut player in &mut player {
        player.walking_time.tick(time.delta());
        if player.is_moving {
            player.height = (3.0
                * (4.0 * player.walking_time.elapsed_secs() % (std::f32::consts::PI + 0.3)).sin())
                as i32;
        }
    }
}

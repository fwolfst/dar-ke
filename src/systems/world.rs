use crate::*;

use std::f32::consts::PI;

pub fn animate(mut giants: Query<&mut Giant>, time: Res<Time>) {
    // animate giant walk
    for mut giant in &mut giants {
        giant.timer.tick(time.delta());
        if giant.timer.finished() {
            giant.frame = (giant.frame + 1).rem_euclid(2);
        }
    }
}

///  tick tickers, animate or despawn.
pub fn update(
    mut giants: Query<&mut Giant>,
    mut player: Query<&mut Player>,
    mut narrative: Query<(Entity, &mut Narrative)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    // Fade out narrative
    for (entity, mut narrative) in &mut narrative {
        narrative.timer.tick(time.delta());
        if narrative.timer.finished() {
            commands.entity(entity).despawn();
        }
    }

    // Head bobble
    // sinus curve over 1 second, 1 second transition to baseline if not moving
    for mut player in &mut player {
        player.walking_time.tick(time.delta());
        // TODO parameterize
        if player.is_moving {
            player.head =
                (3.0 * (4.0 * player.walking_time.elapsed_secs() % (PI + 0.3)).sin()) as i32;
        }
    }
}

pub fn area_effects(mut commands: Commands, time: Res<Time>, mut player: Query<&mut Player>) {
    // if outside bounds, bow
    let mut player = player.single_mut();
    if player.height > 0 {
        //player.height = 10 - time.elapsed_seconds() as i32;
    }
}

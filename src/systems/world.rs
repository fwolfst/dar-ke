use crate::*;

///  tick tickers, animate or despawn.
pub fn update(
    mut giants: Query<&mut Giant>,
    mut narrative: Query<(Entity, &mut Narrative)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for mut giant in &mut giants {
        giant.timer.tick(time.delta());
        if giant.timer.finished() {
            giant.frame = (giant.frame + 1).rem_euclid(2);
        }
    }

    for (entity, mut narrative) in &mut narrative {
        narrative.timer.tick(time.delta());
        if narrative.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

use crate::components::Fading;
use crate::components::Narrative;
use crate::*;

/// Fade out narratives, despawn them once no alpha anymore
pub fn narrative_fading(
    mut narratives: Query<(Entity, &mut Text, &mut Fading), With<Narrative>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (e, mut t, mut f) in narratives.iter_mut() {
        f.timer.tick(time.delta());
        if f.timer.finished() {
            // Fade to invisible or despaen
            t.sections.iter_mut().for_each(|s| {
                s.style.color = s.style.color.with_alpha(1.0 - f.timer.fraction());
                //s.style.color.alpha() - 0.15 * time.delta_seconds());
            });
            if t.sections.iter().any(|s| s.style.color.alpha() <= 0.0) {
                commands.entity(e).despawn();
            }
        }
    }
}

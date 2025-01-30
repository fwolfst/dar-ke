use crate::*;
use components::CameraShake;

use rand::{thread_rng, Rng};

pub fn camera_shake(
    mut shakes: Query<(Entity, &mut CameraShake)>,
    mut camera: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let mut rng = thread_rng();
    for (e, mut shake) in shakes.iter_mut() {
        shake.duration.tick(time.delta());

        if shake.duration.just_finished() {
            commands.entity(e).despawn();
            let mut cam = camera.single_mut();
            cam.translation.x = 0.0;
            cam.translation.y = 0.0;
        } else {
            let mut cam = camera.single_mut();
            let shake_base = 10.0 * shake.strength;
            cam.translation.x += rng.gen_range((-1.0 * shake_base)..shake_base) as f32;
            cam.translation.y += rng.gen_range((-1.0 * shake_base)..shake_base) as f32;
        }
    }
}

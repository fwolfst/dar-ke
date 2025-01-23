use crate::*;

use crate::components::Height;
use crate::components::IsFalling;


pub fn physics(mut fallings: Query<&mut Height, With<IsFalling>>, time: Res<Time>) {
    for mut height in &mut fallings {
        height.height -= 2.0 * time.delta_seconds();
        if height.height < 0.0 {
            height.height = 0.0;
            // remove falling component
        }
    }
}

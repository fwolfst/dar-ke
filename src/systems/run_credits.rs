use crate::*;

use crate::components::CreditRoll;
//use crate::components::Fading;
//use crate::components::Narrative;

pub fn run_credits(
    mut _commands: Commands,
    mut credit_roll: Query<&mut CreditRoll>,
    mut player: Query<&mut Player>,
    time: Res<Time>,
) {
    let mut credit_roll = credit_roll.single_mut();
    let mut player = player.single_mut();
    credit_roll.time.tick(time.delta());

    player.height = -10 + (credit_roll.time.elapsed_secs() / 10.0 * 30.0) as i32;

    //commands.spawn((
    //    components::Narrative,
    //    Text2dBundle {
    //        text: Text {
    //            sections: vec![TextSection::new(
    //                "The end",
    //                TextStyle {
    //                    color: Color::srgb_u8(190, 190, 190),
    //                    ..default()
    //                },
    //            )],
    //            ..Default::default()
    //        },
    //        transform: Transform::from_translation(Vec3::new(0.0, 300.0, 0.0)),
    //        ..default()
    //    },
    //    Fading {
    //        timer: Timer::new(Duration::from_secs(4), TimerMode::Once),
    //    },
    //));
}

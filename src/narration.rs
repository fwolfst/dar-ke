use std::time::Duration;

use bevy::ecs::prelude::*;
use bevy::math::*;
use bevy::prelude::Transform;
use bevy::text::{Text, TextSection, TextStyle};
use bevy::time::{Timer, TimerMode};
use bevy::{color::*, text::Text2dBundle};

use crate::components::{self, Name};

// Missing: lots of strings

/// Simplified parameter to spawn a Narrative"Bundle" (not implemented as one).
pub struct ShowNarrative {
    pub text: String,
    pub color: Color,
    pub position_from_center: Vec2, // center top
    pub fade_time: f32,             // 0.0 -> no fade
}

impl Default for ShowNarrative {
    fn default() -> Self {
        Self {
            text: "Default Narrative Text".into(),
            color: Color::WHITE,
            position_from_center: [0.0, 0.0].into(),
            fade_time: 0f32, // doesnt fade
        }
    }
}

pub fn spawn_narrative(commands: &mut Commands, narration: ShowNarrative) {
    let mut spawned = commands.spawn((
        Name(narration.text.clone()),
        components::Narrative,
        Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    narration.text,
                    TextStyle {
                        color: narration.color,
                        ..bevy::prelude::default()
                    },
                )],
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                narration.position_from_center.x,
                280.0 + narration.position_from_center.y,
                1.0,
            )),
            ..bevy::prelude::default()
        },
    ));

    if narration.fade_time > 0.0 {
        spawned.insert(components::Fading {
            timer: Timer::new(
                Duration::from_secs_f32(narration.fade_time),
                TimerMode::Once,
            ),
        });
    }
}

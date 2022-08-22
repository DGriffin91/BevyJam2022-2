use bevy::{
    ecs::{system::EntityCommands, world::EntityRef},
    prelude::*,
};
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{spawn_from_scene, PlayerCamera};

pub struct ButtonPressEvent {
    pub button_name: String,
    pub button_entity: Entity,
}

#[derive(Clone, Debug, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
#[serde(default)]
pub struct Button {
    pub name: String,
    pub enabled: bool,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            name: Default::default(),
            enabled: true,
        }
    }
}

spawn_from_scene!(button, Button, spawn_hook);

fn spawn_hook(cmds: &mut EntityCommands, _entity: &EntityRef, _trigger: &mut Button) {
    cmds.insert(Collider::cuboid(1.0, 1.0, 1.0)).insert(Sensor);
}

pub(super) fn button_interact_events(
    player_camera: Query<&Transform, With<PlayerCamera>>,
    buttons: Query<&Button>,
    keys: Res<Input<KeyCode>>,
    mut button_press_events: EventWriter<ButtonPressEvent>,
    physics_context: Res<RapierContext>,
) {
    if keys.just_pressed(KeyCode::E) {
        for transform in player_camera.iter() {
            let max_dist = 2.0;

            let ray = physics_context.cast_ray(
                transform.translation,
                transform.forward(),
                max_dist,
                false,
                QueryFilter::default().exclude_solids(), // Only interact with sensors
            );
            if let Some((entity, _)) = ray {
                if let Ok(button) = buttons.get(entity) {
                    if button.enabled {
                        button_press_events.send(ButtonPressEvent {
                            button_name: button.name.clone(),
                            button_entity: entity,
                        });
                    }
                }
            }
        }
    }
}

use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{spawn_from_scene, PlayerCamera};

pub struct ButtonPressEvent {
    pub name: Option<String>,
    pub entity: Entity,
}

/// A button which emits [`ButtonPressEvent`] when pressed.
#[derive(Clone, Debug, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
#[serde(default)]
pub struct Button {
    pub enabled: bool,
}

impl Default for Button {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Default)]
pub struct NamedButtonStatuses(HashMap<String, Entity>);

impl NamedButtonStatuses {
    pub fn any(&self, pat: &str) -> Option<Entity> {
        for (name, status) in &self.0 {
            if name.contains(pat) {
                return Some(*status);
            }
        }
        None
    }
}

spawn_from_scene!(button, Button, |cmds, _entity, _button| {
    cmds.insert(Collider::cuboid(1.0, 1.0, 1.0)).insert(Sensor);
});

pub(super) fn button_interact_events(
    player_camera: Query<&Transform, With<PlayerCamera>>,
    buttons: Query<(Option<&Name>, &Button)>,
    keys: Res<Input<KeyCode>>,
    mut button_press_events: EventWriter<ButtonPressEvent>,
    physics_context: Res<RapierContext>,
    mut named_button_statuses: ResMut<NamedButtonStatuses>,
) {
    named_button_statuses.0 = HashMap::new();
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
                if let Ok((name, button)) = buttons.get(entity) {
                    if button.enabled {
                        let name = name.map(|name| name.to_string());
                        debug!(name = ?name, "Button pressed");
                        button_press_events.send(ButtonPressEvent {
                            name: name.clone(),
                            entity,
                        });
                        if let Some(name) = name {
                            named_button_statuses.0.insert(name, entity);
                        }
                    }
                }
            }
        }
    }
}

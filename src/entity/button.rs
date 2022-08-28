use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{spawn_from_scene, PlayerCamera};

#[derive(Clone)]
pub struct ButtonEvent {
    pub name: Option<String>,
    pub entity: Entity,
    pub pressed: bool,
    pub hovered: bool,
}

/// A button which emits [`ButtonEvent`] when pressed/hovered
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
pub struct NamedButtonStatuses(HashMap<String, ButtonEvent>);

impl NamedButtonStatuses {
    pub fn any(&self, pat: &str) -> Option<ButtonEvent> {
        for (name, event) in &self.0 {
            if name.contains(pat) {
                return Some(event.clone());
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
    mut button_press_events: EventWriter<ButtonEvent>,
    physics_context: Res<RapierContext>,
    mut named_button_statuses: ResMut<NamedButtonStatuses>,
    mouse_button: Res<Input<MouseButton>>,
) {
    let mut new_named_button_statuses = HashMap::new();

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
                    let event = ButtonEvent {
                        name: name.clone(),
                        pressed: mouse_button.just_pressed(MouseButton::Left),
                        hovered: true,
                        entity,
                    };
                    button_press_events.send(event.clone());
                    if let Some(name) = name {
                        new_named_button_statuses.insert(name, event);
                    }
                }
            }
        }
    }

    // Retain buttons that are loosing hover, but were hovered last frame
    named_button_statuses.0.retain(|name, event| {
        if new_named_button_statuses.get(name).is_none() {
            if event.hovered {
                event.hovered = false;
                true
            } else {
                false
            }
        } else {
            false
        }
    });

    named_button_statuses.0.extend(new_named_button_statuses);
}

use bevy::{gltf::GltfExtras, prelude::*};
use bevy_fps_controller::controller::LogicalPlayer;
use serde::Deserialize;

use crate::scene_hook::SceneLoadedEvent;

/// A teleporter component with a destination and activation radius.
#[derive(Component)]
pub struct Teleporter {
    pub destination: Entity,
    pub enabled: bool,
    // TODO: use a box instead of radius
    pub radius: f32,
}

/// A marker component for teleport destinations.
#[derive(Component)]
pub struct TeleporterDestination;

/// Teleport a player if they are within the radius of any active teleporter.
pub(super) fn teleport_player(
    mut player_transforms: Query<&mut Transform, With<LogicalPlayer>>,
    teleporters: Query<(&Transform, &Teleporter), Without<LogicalPlayer>>,
    destinations: Query<&Transform, (With<TeleporterDestination>, Without<LogicalPlayer>)>,
) {
    for mut player_transform in player_transforms.iter_mut() {
        for (teleporter_transform, teleporter) in teleporters.iter() {
            if teleporter.enabled {
                let dist = teleporter_transform
                    .translation
                    .distance(player_transform.translation);

                if dist < teleporter.radius {
                    // Player is in range of teleporter, teleport player
                    if let Ok(destination_transform) = destinations.get(teleporter.destination) {
                        player_transform.translation = destination_transform.translation;
                    }
                }
            }
        }
    }
}

/// Add [`Destination`] component to entities starting with "DESTINATION".
///
/// Destination entities have no custom properties.
pub(super) fn spawn_teleporter_destinations_from_scene(
    world: &World,
    mut cmds: Commands,
    mut scene_loaded_events: EventReader<SceneLoadedEvent>,
    scene_manager: Res<SceneSpawner>,
) {
    for SceneLoadedEvent(instance) in scene_loaded_events.iter() {
        if let Some(entities) = scene_manager.iter_instance_entities(*instance) {
            for entity in entities.filter_map(|e| world.get_entity(e)) {
                if let Some(name) = entity.get::<Name>() {
                    if name.starts_with("DESTINATION") {
                        cmds.entity(entity.id()).insert(TeleporterDestination);
                    }
                }
            }
        }
    }
}

/// Add [`Teleporter`] component to entities starting with "TELEPORTER".
///
/// Teleporter entities should have gltf custom properties which can be deserialized into [`TeleporterGltfExtras`].
pub(super) fn spawn_teleporters_from_scene(
    world: &World,
    mut cmds: Commands,
    mut scene_loaded_events: EventReader<SceneLoadedEvent>,
    entity_names: Query<(Entity, &Name), With<TeleporterDestination>>,
    scene_manager: Res<SceneSpawner>,
) {
    for SceneLoadedEvent(instance) in scene_loaded_events.iter() {
        if let Some(entities) = scene_manager.iter_instance_entities(*instance) {
            for entity in entities.filter_map(|e| world.get_entity(e)) {
                if let Some(name) = entity.get::<Name>() {
                    if name.starts_with("TELEPORTER") {
                        if let Some(extras) = entity.get::<GltfExtras>() {
                            let extras: TeleporterGltfExtras =
                                serde_json::from_str(&extras.value).unwrap();
                            let destination = entity_names.iter().find_map(|(entity, name)| {
                                if **name == extras.destination {
                                    Some(entity)
                                } else {
                                    None
                                }
                            });
                            if let Some(destination) = destination {
                                cmds.entity(entity.id()).insert(Teleporter {
                                    destination,
                                    enabled: extras.enabled,
                                    radius: extras.radius,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(default)]
struct TeleporterGltfExtras {
    destination: String,
    enabled: bool,
    radius: f32,
}

impl Default for TeleporterGltfExtras {
    fn default() -> Self {
        Self {
            destination: "".to_string(),
            enabled: true,
            radius: 3.0,
        }
    }
}

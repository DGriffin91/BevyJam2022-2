use bevy::{gltf::GltfExtras, prelude::*};
use bevy_fps_controller::controller::LogicalPlayer;
use serde::Deserialize;

use crate::scene_hook::SceneLoaded;

/// A teleport component with a destination and activation radius.
#[derive(Component)]
pub struct Teleport {
    pub destination: Entity,
    pub enabled: bool,
    // TODO: use a box instead of radius
    pub radius: f32,
}

/// A marker component for teleport destinations.
#[derive(Component)]
pub struct TeleportDestination;

/// Teleport a player if they are within the radius of any active teleport.
pub(super) fn teleport_player(
    mut player_transforms: Query<&mut Transform, With<LogicalPlayer>>,
    teleports: Query<(&Transform, &Teleport), Without<LogicalPlayer>>,
    destinations: Query<&Transform, (With<TeleportDestination>, Without<LogicalPlayer>)>,
) {
    for mut player_transform in player_transforms.iter_mut() {
        for (teleport_transform, teleport) in teleports.iter() {
            if teleport.enabled {
                let dist = teleport_transform
                    .translation
                    .distance(player_transform.translation);

                if dist < teleport.radius {
                    // Player is in range of teleport, teleport player
                    if let Ok(destination_transform) = destinations.get(teleport.destination) {
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
pub(super) fn spawn_teleport_destinations_from_scene(
    mut cmds: Commands,
    mut scene_loaded: SceneLoaded,
) {
    for entity in scene_loaded.iter() {
        if let Some(name) = entity.get::<Name>() {
            if name.starts_with("DESTINATION ") {
                debug!(
                    name = name.trim_start_matches("DESTINATION"),
                    "Registered teleport destination"
                );
                cmds.entity(entity.id()).insert(TeleportDestination);
            }
        }
    }
}

/// Add [`Teleport`] component to entities starting with "TELEPORT".
///
/// Teleport entities should have gltf custom properties which can be deserialized into [`TeleportDescriptor`].
pub(super) fn spawn_teleports_from_scene(
    mut cmds: Commands,
    mut scene_loaded: SceneLoaded,
    entity_names: Query<(Entity, &Name), With<TeleportDestination>>,
) {
    for entity in scene_loaded.iter() {
        if let Some(name) = entity.get::<Name>() {
            if name.starts_with("TELEPORT ") {
                if let Some(extras) = entity.get::<GltfExtras>() {
                    let extras: TeleportDescriptor = serde_json::from_str(&extras.value).unwrap();
                    let destination = entity_names.iter().find_map(|(entity, name)| {
                        if **name == extras.destination {
                            Some(entity)
                        } else {
                            None
                        }
                    });
                    if let Some(destination) = destination {
                        debug!(name = name.trim_start_matches("TELEPORT"), info = ?extras, "Registered teleport");
                        cmds.entity(entity.id()).insert(Teleport {
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

#[derive(Debug, Deserialize)]
#[serde(default)]
struct TeleportDescriptor {
    destination: String,
    enabled: bool,
    radius: f32,
}

impl Default for TeleportDescriptor {
    fn default() -> Self {
        Self {
            destination: Default::default(),
            enabled: true,
            radius: 3.0,
        }
    }
}

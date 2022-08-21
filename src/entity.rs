pub mod teleporter;

use bevy::prelude::*;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        // Teleporter
        app.add_system(self::teleporter::spawn_teleporter_destinations_from_scene);
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            self::teleporter::spawn_teleporters_from_scene,
        );
        app.add_system(self::teleporter::teleport_player);
    }
}

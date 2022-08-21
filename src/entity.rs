pub mod teleport;

use bevy::prelude::*;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        // Teleport
        app.add_system(self::teleport::spawn_teleport_destinations_from_scene);
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            self::teleport::spawn_teleports_from_scene,
        );
        app.add_system(self::teleport::teleport_player);
    }
}

pub mod door_linear;
pub mod teleport;
pub mod teleport_destination;

use bevy::prelude::*;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        // Door linear
        app.add_system(self::door_linear::spawn_door_linear_from_scene);

        // Teleport
        app.add_system(self::teleport_destination::spawn_teleport_destination_from_scene);
        app.add_system(self::teleport::spawn_teleport_from_scene);
        app.add_system(self::teleport::teleport_player);
    }
}

/// Helper macro to assign a component based on new entities name.
#[macro_export]
macro_rules! spawn_from_scene {
    ($id:ident, $component:ident) => {
        paste::paste! {
            #[doc = "Add [`" $component "`] component to entities starting with \"" $id "\"."]
            pub(super) fn [<spawn_ $id _from_scene>](
                mut cmds: bevy::prelude::Commands,
                mut scene_loaded: $crate::scene_hook::SceneLoaded,
            ) {
                for entity in scene_loaded.iter() {
                    if let Some(name) = entity.get::<bevy::prelude::Name>() {
                        if name.to_lowercase().starts_with(stringify!($id)) {
                            let component: $component = entity
                                .get::<bevy::gltf::GltfExtras>()
                                .map(|extras| {
                                    serde_json::from_str(&extras.value).expect(concat!(
                                        "invalid ",
                                        stringify!($id),
                                        " properties"
                                    ))
                                })
                                .unwrap_or_default();

                            debug!(name = %&name[stringify!($id).len()..].trim(), properties = ?component, concat!("Registered ", stringify!($id)));

                            cmds.entity(entity.id()).insert(component);
                        }
                    }
                }
            }
        }
    };
}

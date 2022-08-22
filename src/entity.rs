pub mod door_linear;
pub mod teleport;
pub mod teleport_destination;

use bevy::prelude::*;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        // Door linear
        app.register_type::<self::door_linear::DoorLinear>();
        app.add_system(self::door_linear::spawn_door_linear_from_scene);
        app.add_system(self::door_linear::update_door);

        // Teleport
        app.register_type::<self::teleport::Teleport>();
        app.add_system(self::teleport::spawn_teleport_from_scene);
        app.add_system(self::teleport::teleport_player);

        // Teleport destination
        app.register_type::<self::teleport_destination::TeleportDestination>();
        app.add_system(self::teleport_destination::spawn_teleport_destination_from_scene);
    }
}

/// Helper macro to assign a component based on new entities name.
#[macro_export]
macro_rules! spawn_from_scene {
    ($id:ident, $component:ident $(, $func:ident)?) => {
        paste::paste! {
            #[doc = "Add [`" $component "`] component to entities containing uppercase \"" $id "\"."]
            pub(super) fn [<spawn_ $id _from_scene>](
                mut cmds: bevy::prelude::Commands,
                mut scene_loaded: $crate::scene_hook::SceneLoaded,
            ) {
                for entity in scene_loaded.iter() {
                    if let Some(name) = entity.get::<bevy::prelude::Name>() {
                        if name.contains(&stringify!($id).to_uppercase()) {
                            #[allow(unused_mut)]
                            let mut component: $component = entity
                                .get::<bevy::gltf::GltfExtras>()
                                .map(|extras| {
                                    serde_json::from_str(&extras.value).expect(concat!(
                                        "invalid ",
                                        stringify!($id),
                                        " properties"
                                    ))
                                })
                                .unwrap_or_default();

                            debug!(id = ?entity.id(), name = %&name[stringify!($id).len()..].trim(), properties = ?component, concat!("Registered ", stringify!($id)));

                            $ ( $func( &mut cmds, &entity, &mut component ); )?
                            cmds.entity(entity.id()).insert(component);

                        }
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! entities_containing_name {
    ($contains:expr, $names:ident) => {
        $names
            .iter()
            .filter(|(_, n)| n.contains($contains))
            .map(|(e, _)| e)
    };
}

pub fn entity_name_contains(
    contains: &str,
    entity: Entity,
    names: &Query<(Entity, &Name)>,
) -> bool {
    if names.get(entity).unwrap().1.contains(contains) {
        return true;
    }
    false
}

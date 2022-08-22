pub mod block;
pub mod button;
pub mod collider;
pub mod door_linear;
pub mod teleport;
pub mod teleport_destination;
pub mod trigger;

use bevy::prelude::*;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        // Block
        app.register_type::<self::block::Block>();
        app.add_system(self::block::spawn_block_from_scene);

        // Button
        app.register_type::<self::button::Button>();
        app.add_event::<self::button::ButtonPressEvent>();
        app.add_system(self::button::spawn_button_from_scene);
        app.add_system(self::button::button_interact_events);

        // Collider
        app.register_type::<self::collider::Collider>();
        app.add_system(self::collider::spawn_collider_from_scene);

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

        // Trigger
        app.register_type::<self::trigger::Trigger>();
        app.add_event::<self::trigger::TriggerEnterEvent>();
        app.add_event::<self::trigger::TriggerExitEvent>();
        app.add_system(self::trigger::spawn_trigger_from_scene);
        app.add_system(self::trigger::trigger_collision_events);
    }
}

/// Helper macro to assign a component based on new entities name.
#[macro_export]
macro_rules! spawn_from_scene {
    (
        $id:ident,
        $component:ident
        $(, |$cmds:ident, $entity:ident, $comp:ident $(, $( $args:ident : $arg_ty:ty ),* )? | { $( $body:tt )* } )?
    ) => {
        paste::paste! {
            #[doc = "Add [`" $component "`] component to entities containing uppercase \"" $id "\"."]
            pub(super) fn [<spawn_ $id _from_scene>](
                mut cmds: bevy::prelude::Commands,
                mut scene_loaded: $crate::scene_hook::SceneLoaded,
                $( $( $( #[allow(unused_mut)] mut $args: $arg_ty ),* )? )?
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

                            let mut entity_cmd = cmds.entity(entity.id());
                            $({
                                let $entity = entity;
                                let $cmds = &mut entity_cmd;
                                let $comp = &mut component;

                                $( $body )*
                            })?
                            entity_cmd.insert(component);
                        }
                    }
                }
            }
        }
    };
}

pub trait NamedItems<T>
where
    T: bevy::ecs::query::WorldQuery,
{
    fn find_named(
        &self,
        name: &str,
    ) -> Option<<<<T as bevy::ecs::query::WorldQuery>::ReadOnly as bevy::ecs::query::WorldQueryGats<'_>>::Fetch as bevy::ecs::query::Fetch<'_>>::Item>;
}

macro_rules! impl_named_items {
    ($t:ident $(, $types:ident )*) => {
        impl<'w, 's, $t $(, $types )* , Filter> NamedItems<$t> for Query<'w, 's, (&Name, $t $(, $types )*), Filter>
        where
            $t: bevy::ecs::query::WorldQuery,
            $( $types: bevy::ecs::query::WorldQuery, )*
            Filter: bevy::ecs::query::WorldQuery,
        {
            fn find_named(
                &self,
                name: &str,
            ) -> Option<<<<$t as bevy::ecs::query::WorldQuery>::ReadOnly as bevy::ecs::query::WorldQueryGats<'_>>::Fetch as bevy::ecs::query::Fetch<'_>>::Item>
            {
                self.iter().find_map(|(entity_name, value, ..)| {
                    if entity_name.contains(name) {
                        Some(value)
                    } else {
                        None
                    }
                })
            }
        }
    };
}

impl_named_items!(A);
impl_named_items!(A, B);
impl_named_items!(A, B, C);
impl_named_items!(A, B, C, D);
impl_named_items!(A, B, C, D, E);
impl_named_items!(A, B, C, D, E, F);
impl_named_items!(A, B, C, D, E, F, G);
impl_named_items!(A, B, C, D, E, F, G, H);

pub trait NamedItemsMut<T>
where
    T: bevy::ecs::query::WorldQuery,
{
    fn find_named_mut(
        &mut self,
        name: &str,
    ) -> Option<
        <<T as bevy::ecs::query::WorldQueryGats<'_>>::Fetch as bevy::ecs::query::Fetch<'_>>::Item,
    >;
}

macro_rules! impl_named_items_mut {
    ($t:ident $(, $types:ident )*) => {
        impl<'w, 's, $t $(, $types )* , Filter> NamedItemsMut<$t> for Query<'w, 's, (&Name, $t $(, $types )*), Filter>
        where
            $t: bevy::ecs::query::WorldQuery,
            $( $types: bevy::ecs::query::WorldQuery, )*
            Filter: bevy::ecs::query::WorldQuery,
        {
            fn find_named_mut(
                &mut self,
                name: &str,
            ) -> Option<<<$t as bevy::ecs::query::WorldQueryGats<'_>>::Fetch as bevy::ecs::query::Fetch<'_>>::Item> {
                self.iter_mut().find_map(|(entity_name, value, ..)| {
                    if entity_name.contains(name) {
                        Some(value)
                    } else {
                        None
                    }
                })
            }
        }
    };
}

impl_named_items_mut!(A);
impl_named_items_mut!(A, B);
impl_named_items_mut!(A, B, C);
impl_named_items_mut!(A, B, C, D);
impl_named_items_mut!(A, B, C, D, E);
impl_named_items_mut!(A, B, C, D, E, F);
impl_named_items_mut!(A, B, C, D, E, F, G);
impl_named_items_mut!(A, B, C, D, E, F, G, H);

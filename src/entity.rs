pub mod block;
pub mod button;
pub mod collider;
pub mod door_linear;
pub mod teleport;
pub mod teleport_destination;
pub mod trigger;

use bevy::prelude::*;

use crate::{impl_named_items, impl_named_items_mut};

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

pub trait NamedItems<'a, T> {
    fn find_named(&'a mut self, name: &str) -> Option<T>;
}

impl_named_items!(A);
impl_named_items!(A, B);
impl_named_items!(A, B, C);
impl_named_items!(A, B, C, D);
impl_named_items!(A, B, C, D, E);
impl_named_items!(A, B, C, D, E, F);
impl_named_items!(A, B, C, D, E, F, G);
impl_named_items!(A, B, C, D, E, F, G, H);

pub trait NamedItemsMut<'a, T> {
    fn find_named_mut(&'a mut self, name: &str) -> Option<T>;
}

impl_named_items_mut!(A);
impl_named_items_mut!(A, B);
impl_named_items_mut!(A, B, C);
impl_named_items_mut!(A, B, C, D);
impl_named_items_mut!(A, B, C, D, E);
impl_named_items_mut!(A, B, C, D, E, F);
impl_named_items_mut!(A, B, C, D, E, F, G);
impl_named_items_mut!(A, B, C, D, E, F, G, H);

pub mod block;
pub mod button;
pub mod collider;
pub mod door_linear;
pub mod teleport;
pub mod teleport_destination;
pub mod trigger;

use bevy::prelude::*;

use crate::impl_named;

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

pub trait Named {
    fn name(&self) -> Option<&str>;
}

impl Named for &Name {
    fn name(&self) -> Option<&str> {
        Some(self.as_str())
    }
}

impl_named!(A);
impl_named!(A, B);
impl_named!(A, B, C);
impl_named!(A, B, C, D);
impl_named!(A, B, C, D, E);
impl_named!(A, B, C, D, E, F);
impl_named!(A, B, C, D, E, F, G);

pub struct NamedFilterMap<'a, I> {
    iter: I,
    name: &'a str,
}

impl<'a, I: Iterator> Iterator for NamedFilterMap<'a, I>
where
    I::Item: Named,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        self.iter.find_map(|item| {
            if let Some(item_name) = item.name() {
                if item_name.contains(self.name) {
                    Some(item)
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }
}

pub trait NamedIterator<I> {
    fn filter_name_contains(self, name: &str) -> NamedFilterMap<I>;
}

impl<T, I> NamedIterator<T> for T
where
    T: Iterator<Item = I>,
    I: Named,
{
    fn filter_name_contains(self, name: &str) -> NamedFilterMap<T> {
        NamedFilterMap { iter: self, name }
    }
}

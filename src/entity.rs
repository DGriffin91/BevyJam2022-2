pub mod block;
pub mod button;
pub mod collider;
pub mod door_linear;
pub mod phone;
pub mod teleport;
pub mod teleport_destination;
pub mod trigger;

use bevy::prelude::*;

use crate::{impl_named, register_entity};

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        // Block
        register_entity!(app, block);

        // Button
        register_entity!(
            app,
            button,
            events = [ButtonEvent],
            resources = [NamedButtonStatuses],
            systems = [button_interact_events]
        );

        // Collider
        register_entity!(app, collider);

        // Door linear
        register_entity!(
            app,
            door_linear,
            events = [DoorFullyClosedEvent, DoorFullyOpenedEvent],
            resources = [NamedDoorStatuses],
            systems = [door_sounds, update_door]
        );

        // Phone
        register_entity!(
            app,
            phone,
            events = [
                PhoneMenuOpenEvent,
                PhoneMenuCloseEvent,
                PhoneDigitEnterEvent,
                PhoneSubmitEvent
            ],
            resources = [PhoneUiEnabled, PhoneUiVisible],
            systems = [
                number_not_availble,
                press_phone_keys,
                phone_interact_events,
                resize_phone_ui,
                sync_phone_visibility
            ],
            startup_systems = [setup_phone_ui]
        );

        // Teleport destination
        register_entity!(app, teleport_destination);

        // Teleport
        register_entity!(app, teleport, systems = [teleport_player]);

        // Trigger
        register_entity!(
            app,
            trigger,
            events = [TriggerEnterEvent, TriggerExitEvent],
            resources = [NamedTriggerStatuses],
            systems = [trigger_collision_events]
        );
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

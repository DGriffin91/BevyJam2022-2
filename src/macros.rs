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

/// Implements [`NamedItems`] for queries.
#[macro_export]
macro_rules! impl_named_items {
    ($t:ident $(, $types:ident )*) => {
        impl<'w, 's, 'a, $t $(, $types )* , Filter> $crate::entity::NamedItems<'a, <<<$t as bevy::ecs::query::WorldQuery>::ReadOnly as bevy::ecs::query::WorldQueryGats<'a>>::Fetch as bevy::ecs::query::Fetch<'a>>::Item> for Query<'w, 's, (&Name, $t $(, $types )*), Filter>
        where
            $t: bevy::ecs::query::WorldQuery,
            $( $types: bevy::ecs::query::WorldQuery, )*
            Filter: bevy::ecs::query::WorldQuery,
        {
            fn find_named(
                &mut self,
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

/// Implements [`NamedItemsMut`] for queries.
#[macro_export]
macro_rules! impl_named_items_mut {
    ($t:ident $(, $types:ident )*) => {
        impl<'w, 's, 'a, $t $(, $types )* , Filter> $crate::entity::NamedItemsMut<'a, <<$t as bevy::ecs::query::WorldQueryGats<'a>>::Fetch as bevy::ecs::query::Fetch<'a>>::Item> for Query<'w, 's, (&Name, $t $(, $types )*), Filter>
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

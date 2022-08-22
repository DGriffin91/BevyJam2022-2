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

/// Helper macro to assign a component based on new entities name.
#[macro_export]
macro_rules! impl_named {
    ($( $args:ident ),*) => {
        impl<$( $args ),*> Named for (&Name $(, $args )*) {
            fn name(&self) -> Option<&str> {
                Some(self.0.as_str())
            }
        }
    };
}

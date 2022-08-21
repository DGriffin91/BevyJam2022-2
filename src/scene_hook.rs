//! Systems to insert components on loaded scenes.
//!
//! Please see the [`SceneHook`] documentation for detailed examples.

use bevy::{
    ecs::system::{SystemMeta, SystemParamFetch, SystemParamState},
    scene::{SceneInstance, SceneSpawner},
};
use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        event::ManualEventReader,
        prelude::{Without, World},
        schedule::ShouldRun,
        system::{Commands, EntityCommands, LocalState, Query, Res, ResState},
        world::EntityRef,
    },
    scene::InstanceId,
};

use bevy::{ecs::system::SystemParam, prelude::*};

/// Marker Component for scenes that were hooked.
#[derive(Component, Debug)]
#[non_exhaustive]
pub struct SceneHooked;

pub struct SceneLoadedEvent(pub InstanceId);

/// Add this as a component to any entity to run `hook`
/// when the scene is loaded.
///
/// You can use it to add your own non-serializable components to entites
/// present in a scene file.
///
/// A typical usage is adding animation, physics collision data or marker
/// components to a scene spawned from a file format that do not support it.
///
/// # Example
///
///  ```rust
/// # use bevy::ecs::{system::Res, component::Component, system::Commands};
/// # use bevy::asset::AssetServer;
/// # use bevy::utils::default;
/// # use bevy::scene::SceneBundle;
/// use bevy_scene_hook::{SceneHook, HookedSceneBundle};
/// # #[derive(Component)]
/// # struct Name; impl Name { fn as_str(&self) -> &str { todo!() } }
/// enum PileType { Drawing }
///
/// #[derive(Component)]
/// struct Pile(PileType);
///
/// #[derive(Component)]
/// struct Card;
///
/// fn load_scene(mut cmds: Commands, asset_server: Res<AssetServer>) {
///     cmds.spawn_bundle(HookedSceneBundle {
///         scene: SceneBundle { scene: asset_server.load("scene.glb#Scene0"), ..default() },
///         hook: SceneHook::new(|entity, cmds| {
///             match entity.get::<Name>().map(|t|t.as_str()) {
///                 Some("Pile") => cmds.insert(Pile(PileType::Drawing)),
///                 Some("Card") => cmds.insert(Card),
///                 _ => cmds,
///             };
///         }),
///     });
/// }
/// ```
#[derive(Component)]
pub struct SceneHook {
    hook: Box<dyn Fn(&EntityRef, &World, &mut EntityCommands) + Send + Sync + 'static>,
}
impl SceneHook {
    /// Add a hook to a scene, to run for each entities when the scene is
    /// loaded.
    ///
    /// The hook adds [`Component`]s or do anything with entity in the spawned
    /// scene refered by `EntityRef`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use bevy::ecs::{
    ///     world::EntityRef, component::Component,
    ///     system::{Commands, Res, EntityCommands}
    /// # };
    /// # use bevy::asset::{AssetServer, Handle};
    /// # use bevy::utils::default;
    /// # use bevy::scene::{Scene, SceneBundle};
    /// use bevy_scene_hook::{SceneHook, HookedSceneBundle};
    /// # #[derive(Component)] struct Name;
    /// # type DeckData = Scene;
    /// #[derive(Clone)]
    /// struct DeckAssets { player: Handle<DeckData>, oppo: Handle<DeckData> }
    ///
    /// fn hook(decks: &DeckAssets, entity: &EntityRef, cmds: &mut EntityCommands) {}
    /// fn load_scene(mut cmds: Commands, decks: Res<DeckAssets>, assets: Res<AssetServer>) {
    ///     let decks = decks.clone();
    ///     cmds.spawn_bundle(HookedSceneBundle {
    ///         scene: SceneBundle { scene: assets.load("scene.glb#Scene0"), ..default() },
    ///         hook: SceneHook::new(move |entity, cmds| hook(&decks, entity, cmds)),
    ///     });
    /// }
    /// ```
    pub fn new<F: Fn(&EntityRef, &World, &mut EntityCommands) + Send + Sync + 'static>(
        hook: F,
    ) -> Self {
        Self {
            hook: Box::new(hook),
        }
    }
}

/// Run once [`SceneHook`]s added to [`SceneBundle`](crate::SceneBundle) or
/// [`DynamicSceneBundle`](crate::DynamicSceneBundle) when the scenes are loaded.
pub fn run_hooks(
    unloaded_instances: Query<(Entity, &SceneInstance, &SceneHook), Without<SceneHooked>>,
    scene_manager: Res<SceneSpawner>,
    world: &World,
    mut cmds: Commands,
) {
    for (entity, instance, hooked) in unloaded_instances.iter() {
        if let Some(entities) = scene_manager.iter_instance_entities(**instance) {
            for entity_ref in entities.filter_map(|e| world.get_entity(e)) {
                let mut cmd = cmds.entity(entity_ref.id());
                (hooked.hook)(&entity_ref, world, &mut cmd);
            }
            cmds.entity(entity).insert(SceneHooked);
        }
    }
}

pub fn scene_loaded_events_system(
    added_scenes: Query<&SceneInstance, Added<SceneInstance>>,
    mut scene_loaded_events: EventWriter<SceneLoadedEvent>,
) {
    for scene in added_scenes.iter() {
        let instance = **scene;
        scene_loaded_events.send(SceneLoadedEvent(instance));
    }
}

#[derive(Bundle)]
pub struct HookedSceneBundle {
    pub hook: SceneHook,
    #[bundle]
    pub scene: SceneBundle,
}

/// Convenience parameter to query if a scene marked with `M` has been loaded.
#[derive(SystemParam)]
pub struct HookedSceneState<'w, 's, M: Component> {
    query: Query<'w, 's, (), (With<M>, With<SceneHooked>)>,
}
impl<'w, 's, T: Component> HookedSceneState<'w, 's, T> {
    #[allow(dead_code)]
    pub fn is_loaded(&self) -> bool {
        self.query.iter().next().is_some()
    }
}

/// Convenience run criteria to query if a scene marked with `M` has been loaded.
#[allow(dead_code)]
pub fn is_scene_hooked<M: Component>(state: HookedSceneState<M>) -> ShouldRun {
    match state.is_loaded() {
        true => ShouldRun::Yes,
        false => ShouldRun::No,
    }
}

/// Systems defined in the [`bevy_scene_hook`](crate) crate (this crate).
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum Systems {
    /// System running the hooks.
    SceneHookRunner,
}

/// Plugin to run hooks associated with spawned scenes.
pub struct HookPlugin;
impl Plugin for HookPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SceneLoadedEvent>();
        app.add_system(run_hooks.label(Systems::SceneHookRunner));
        app.add_system_to_stage(
            CoreStage::PreUpdate,
            scene_loaded_events_system.label(Systems::SceneHookRunner),
        );
    }
}

pub struct SceneLoaded<'w, 's> {
    world: &'w World,
    reader: Local<'s, ManualEventReader<SceneLoadedEvent>>,
    events: Res<'w, Events<SceneLoadedEvent>>,
    scene_manager: Res<'w, SceneSpawner>,
}

impl<'w, 's> SceneLoaded<'w, 's> {
    pub fn iter(&mut self) -> impl Iterator<Item = EntityRef<'_>> {
        self.reader
            .iter_with_id(&self.events)
            .filter_map(|(event, _id)| {
                self.scene_manager
                    .iter_instance_entities(event.0)
                    .map(|entities| entities.filter_map(|e| self.world.get_entity(e)))
            })
            .flatten()
    }
}

impl<'w, 's> SystemParam for SceneLoaded<'w, 's> {
    type Fetch = SceneLoadedState;
}

pub struct SceneLoadedState {
    reader: LocalState<ManualEventReader<SceneLoadedEvent>>,
    events: ResState<Events<SceneLoadedEvent>>,
    scene_manager: ResState<SceneSpawner>,
}

unsafe impl SystemParamState for SceneLoadedState {
    fn init(world: &mut World, system_meta: &mut SystemMeta) -> Self {
        let reader = LocalState::<ManualEventReader<SceneLoadedEvent>>::init(world, system_meta);
        let events = ResState::<Events<SceneLoadedEvent>>::init(world, system_meta);
        let scene_manager = ResState::<SceneSpawner>::init(world, system_meta);

        SceneLoadedState {
            reader,
            events,
            scene_manager,
        }
    }
}

impl<'w, 's> SystemParamFetch<'w, 's> for SceneLoadedState {
    type Item = SceneLoaded<'w, 's>;

    unsafe fn get_param(
        state: &'s mut Self,
        system_meta: &bevy::ecs::system::SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item {
        let reader = SystemParamFetch::<'w, 's>::get_param(
            &mut state.reader,
            system_meta,
            world,
            change_tick,
        );
        let events = SystemParamFetch::<'w, 's>::get_param(
            &mut state.events,
            system_meta,
            world,
            change_tick,
        );
        let scene_manager = SystemParamFetch::<'w, 's>::get_param(
            &mut state.scene_manager,
            system_meta,
            world,
            change_tick,
        );

        SceneLoaded {
            world,
            reader,
            events,
            scene_manager,
        }
    }
}

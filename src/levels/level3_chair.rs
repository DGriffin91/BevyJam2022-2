use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    assets::ModelAssets,
    materials::rings::RingsMaterial,
    scene_hook::{HookedSceneBundle, SceneHook},
};

use super::Level;

pub struct Level3ChairPlugin;
impl Plugin for Level3ChairPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(Level::Level3Chair, setup.after("pre_process"));
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(Level::Level3Chair)
                .with_system(rotate_rings)
                .into(),
        );
    }
}

fn setup(mut cmds: Commands, model_assets: Res<ModelAssets>) {
    cmds.spawn_bundle(HookedSceneBundle {
        scene: SceneBundle {
            scene: model_assets.level3_chair.clone(),
            ..default()
        },
        hook: SceneHook::new(move |_entity, _world, cmds| {
            cmds.insert(Level::Level3Chair);
        }),
    });
}

#[derive(Default)]
pub struct RingsSetup {
    pub direction: bool,
    pub speed: bool,
    pub color: bool,
}

impl RingsSetup {
    #[allow(unused)]
    fn is_correct(&self) -> bool {
        self.direction && !self.speed && self.color
    }
}

fn rotate_rings(
    time: Res<Time>,
    rings_setup: Res<RingsSetup>,
    mut rings: Query<(&mut Transform, &Name, &Handle<RingsMaterial>)>,
    mut rings_mats: ResMut<Assets<RingsMaterial>>,
) {
    for (mut transform, name, material) in &mut rings {
        let speed = if name.contains("Ring1") {
            if let Some(mat) = rings_mats.get_mut(material) {
                let color = if rings_setup.color {
                    Color::rgba(1.0, 0.0, 1.0, 1.0)
                } else {
                    Color::rgba(1.0, 0.4, 0.03, 1.0)
                };
                if mat.base_color != color {
                    mat.base_color = color;
                }
            }
            let mut speed = if rings_setup.speed { 1.0 } else { 1.5 };
            if !rings_setup.direction {
                speed = -speed;
            }
            speed
        } else {
            1.5
        };
        //transform.rotation =
        //    Quat::from_xyzw(0.6, speed * time.seconds_since_startup() as f32, 1.1, 0.0);
        transform.rotate_x(speed * time.delta_seconds());
        transform.rotate_z(0.8 * speed * time.delta_seconds());
    }
}

use bevy::prelude::*;

use bevy_rapier3d::prelude::*;

use crate::PlayerCamera;

pub struct InteractPlugin;

impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractEvent>();
        app.add_system(interaction);
        app.add_system(interact_debug);
        app.init_resource::<InteractDebugMode>();
    }
}
#[derive(Default)]
pub struct InteractDebugMode(pub bool);
pub struct InteractEvent(pub Entity);

pub fn interaction(
    player_camera: Query<&Transform, With<PlayerCamera>>,
    keys: Res<Input<KeyCode>>,
    mut interact_events: EventWriter<InteractEvent>,
    physics_context: Res<RapierContext>,
) {
    if keys.just_pressed(KeyCode::E) {
        for transform in player_camera.iter() {
            let max_dist = 2.0;

            let ray = physics_context.cast_ray(
                transform.translation,
                transform.forward(),
                max_dist,
                false,
                QueryFilter::default().exclude_solids(), // Only interact with sensors
            );
            if let Some((e, _)) = ray {
                interact_events.send(InteractEvent(e));
            }
        }
    }
}

pub fn debug_event_print(
    msg: &str,
    entity: Entity,
    transforms: &Query<&Transform>,
    names: &Query<&Name>,
) {
    // position is just from origin of other entity currently
    let mut pos_text = String::new();
    if let Ok(t) = transforms.get(entity) {
        pos_text = format!("{:?}", t.translation)
    }
    println!(
        "{} with {:?} name {} at {}",
        msg,
        entity,
        names.get(entity).unwrap_or(&Name::default()),
        pos_text
    )
}

pub fn interact_debug(
    debug_mode: Res<InteractDebugMode>,
    mut interact_events: EventReader<InteractEvent>,
    transforms: Query<&Transform>,
    names: Query<&Name>,
) {
    if debug_mode.0 {
        for event in interact_events.iter() {
            debug_event_print("InteractEvent", event.0, &transforms, &names);
        }
    }
}

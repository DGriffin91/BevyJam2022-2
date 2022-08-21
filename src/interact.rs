use bevy::prelude::*;
use bevy_fps_controller::controller::{FpsController, LogicalPlayer};
use bevy_rapier3d::prelude::*;

pub struct InteractPlugin;

impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractEvent>();
        app.add_system(interaction);
    }
}

pub struct InteractEvent(pub Entity);

pub fn interaction(
    player: Query<(Entity, &Transform, &FpsController), With<LogicalPlayer>>,
    keys: Res<Input<KeyCode>>,
    mut interact_events: EventWriter<InteractEvent>,
    physics_context: Res<RapierContext>,
) {
    if keys.just_pressed(KeyCode::E) {
        for (entity, transform, controller) in player.iter() {
            let origin = transform.translation;
            let look_dir =
                Quat::from_euler(EulerRot::ZYX, 0.0, controller.yaw, controller.pitch) * -Vec3::Z;
            let max_dist = 2.0;
            let groups = QueryFilter::default().exclude_rigid_body(entity);
            let ray = physics_context.cast_ray(origin, look_dir, max_dist, false, groups);
            if let Some((e, _)) = ray {
                interact_events.send(InteractEvent(e));
            }
        }
    }
}

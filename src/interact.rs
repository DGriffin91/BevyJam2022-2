use bevy::prelude::*;

use bevy_rapier3d::prelude::*;

use crate::PlayerCamera;

pub struct InteractPlugin;

impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractEvent>();
        app.add_system(interaction);
    }
}

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

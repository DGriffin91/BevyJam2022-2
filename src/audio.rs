use std::time::Duration;

use bevy::{prelude::*, utils::Uuid};
use bevy_kira_audio::prelude::{AudioControl, DynamicAudioChannel, DynamicAudioChannels};
use iyes_loopless::prelude::*;

use crate::PlayerCamera;

pub struct AudioComponentPlugin;

impl Plugin for AudioComponentPlugin {
    fn build(&self, app: &mut App) {
        let mut audio_stage = SystemStage::parallel();
        audio_stage.add_system(pan);

        app.add_stage_before(
            CoreStage::Update,
            "audio_fixed_update",
            FixedTimestepStage::new(Duration::from_millis(32)) // 30 fps
                .with_stage(audio_stage),
        );
    }
}

#[derive(Component, Clone)]
pub struct AudioComponent(String);

impl AudioComponent {
    pub fn new(channels: &mut DynamicAudioChannels) -> AudioComponent {
        let id = Uuid::new_v4().to_string();
        channels.create_channel(&id);
        AudioComponent(id)
    }
    pub fn channel<'a>(&'a self, channels: &'a DynamicAudioChannels) -> &DynamicAudioChannel {
        channels.channel(&self.0)
    }
    pub fn get_or_insert<'a>(
        cmds: &mut Commands,
        entity: Entity,
        component: Option<&AudioComponent>,
        channels: &'a mut DynamicAudioChannels,
    ) -> AudioComponent {
        if let Some(c) = component {
            c.clone()
        } else {
            let c = AudioComponent::new(channels);
            cmds.entity(entity).insert(c.clone());
            c
        }
    }
}

fn pan(
    audio: Res<DynamicAudioChannels>,
    query: Query<(&Transform, &AudioComponent)>,
    player_camera: Query<&Transform, With<PlayerCamera>>,
) {
    if let Some(cam_trans) = player_camera.iter().next() {
        for (aud_trans, ch) in &query {
            let ch = ch.channel(&audio);
            let camera_to_aud = (aud_trans.translation - cam_trans.translation).normalize_or_zero();
            // Get dot product between camera right vector, and vector pointing from camera to sound src
            // When pointing at sound src dot is 0.0, when pointing right dot is 1.0, left is -1.0
            let mut pan = cam_trans.right().dot(camera_to_aud);
            pan = pan * 0.5 + 0.5; // pan input expects 0.0 to 1.0
            ch.set_panning(pan as f64);

            let dist = aud_trans.translation.distance(cam_trans.translation) * 0.1;
            let level = (1.0 / dist).min(1.0) * 0.1; // not accurate falloff
            ch.set_volume(level as f64);
        }
    }
}

use std::time::Duration;

use bevy::{prelude::*, window::WindowResized};

use bevy_kira_audio::{prelude::Audio, AudioControl, AudioInstance, AudioTween};
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    assets::{FontAssets, ImageAssets, SoundAssets},
    get_display_scale,
    notification::NotificationText,
    spawn_from_scene, PlayerCamera,
};

pub struct PhoneMenuOpenEvent {
    pub name: Option<String>,
    pub entity: Entity,
}

pub struct PhoneMenuCloseEvent {
    pub name: Option<String>,
    pub entity: Entity,
}

pub struct PhoneDigitEnterEvent {
    pub name: Option<String>,
    pub entity: Entity,
    pub digit: u8,
}

pub struct PhoneSubmitEvent {
    // pub name: Option<String>,
    // pub entity: Entity,
    pub number: String,
}

/// A phone which can have a number inputted.
#[derive(Clone, Debug, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Phone {
    pub enabled: bool,
    pub number: u32,
}

impl Default for Phone {
    fn default() -> Self {
        Self {
            enabled: true,
            number: 0,
        }
    }
}

spawn_from_scene!(phone, Phone);

#[derive(Clone, Copy, Debug, Default, Deref, DerefMut, PartialEq, Eq)]
pub struct PhoneUiVisible(pub bool);

#[derive(Clone, Copy, Debug, Deref, DerefMut, PartialEq, Eq)]
pub struct PhoneUiEnabled(pub bool);

impl Default for PhoneUiEnabled {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Clone, Copy, Component, PartialEq, Eq)]
pub(super) enum PhoneKey {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyHash,
    KeyAsterix,
}

impl PhoneKey {
    fn char(&self) -> char {
        match self {
            PhoneKey::Key0 => '0',
            PhoneKey::Key1 => '1',
            PhoneKey::Key2 => '2',
            PhoneKey::Key3 => '3',
            PhoneKey::Key4 => '4',
            PhoneKey::Key5 => '5',
            PhoneKey::Key6 => '6',
            PhoneKey::Key7 => '7',
            PhoneKey::Key8 => '8',
            PhoneKey::Key9 => '9',
            PhoneKey::KeyHash => '#',
            PhoneKey::KeyAsterix => '*',
        }
    }

    fn image(&self, image_assets: &ImageAssets) -> Handle<Image> {
        match self {
            PhoneKey::Key0 => image_assets.phone_key_0.clone(),
            PhoneKey::Key1 => image_assets.phone_key_1.clone(),
            PhoneKey::Key2 => image_assets.phone_key_2.clone(),
            PhoneKey::Key3 => image_assets.phone_key_3.clone(),
            PhoneKey::Key4 => image_assets.phone_key_4.clone(),
            PhoneKey::Key5 => image_assets.phone_key_5.clone(),
            PhoneKey::Key6 => image_assets.phone_key_6.clone(),
            PhoneKey::Key7 => image_assets.phone_key_7.clone(),
            PhoneKey::Key8 => image_assets.phone_key_8.clone(),
            PhoneKey::Key9 => image_assets.phone_key_9.clone(),
            PhoneKey::KeyHash => image_assets.phone_key_hash.clone(),
            PhoneKey::KeyAsterix => image_assets.phone_key_asterix.clone(),
        }
    }

    fn image_pressed(&self, image_assets: &ImageAssets) -> Handle<Image> {
        match self {
            PhoneKey::Key0 => image_assets.phone_key_0_pressed.clone(),
            PhoneKey::Key1 => image_assets.phone_key_1_pressed.clone(),
            PhoneKey::Key2 => image_assets.phone_key_2_pressed.clone(),
            PhoneKey::Key3 => image_assets.phone_key_3_pressed.clone(),
            PhoneKey::Key4 => image_assets.phone_key_4_pressed.clone(),
            PhoneKey::Key5 => image_assets.phone_key_5_pressed.clone(),
            PhoneKey::Key6 => image_assets.phone_key_6_pressed.clone(),
            PhoneKey::Key7 => image_assets.phone_key_7_pressed.clone(),
            PhoneKey::Key8 => image_assets.phone_key_8_pressed.clone(),
            PhoneKey::Key9 => image_assets.phone_key_9_pressed.clone(),
            PhoneKey::KeyHash => image_assets.phone_key_hash_pressed.clone(),
            PhoneKey::KeyAsterix => image_assets.phone_key_asterix_pressed.clone(),
        }
    }

    fn matches_key(&self, keycode: KeyCode) -> bool {
        match keycode {
            KeyCode::Key0 | KeyCode::Numpad0 if self == &PhoneKey::Key0 => true,
            KeyCode::Key1 | KeyCode::Numpad1 if self == &PhoneKey::Key1 => true,
            KeyCode::Key2 | KeyCode::Numpad2 if self == &PhoneKey::Key2 => true,
            KeyCode::Key3 | KeyCode::Numpad3 if self == &PhoneKey::Key3 => true,
            KeyCode::Key4 | KeyCode::Numpad4 if self == &PhoneKey::Key4 => true,
            KeyCode::Key5 | KeyCode::Numpad5 if self == &PhoneKey::Key5 => true,
            KeyCode::Key6 | KeyCode::Numpad6 if self == &PhoneKey::Key6 => true,
            KeyCode::Key7 | KeyCode::Numpad7 if self == &PhoneKey::Key7 => true,
            KeyCode::Key8 | KeyCode::Numpad8 if self == &PhoneKey::Key8 => true,
            KeyCode::Key9 | KeyCode::Numpad9 if self == &PhoneKey::Key9 => true,
            _ => false,
        }
    }
}

pub(super) fn test_toggle_phone(
    keys: Res<Input<KeyCode>>,
    mut phone_ui_visible: ResMut<PhoneUiVisible>,
) {
    if keys.just_pressed(KeyCode::F) {
        **phone_ui_visible = !**phone_ui_visible;
    }
}

pub(super) fn resize_phone_ui(
    mut ui_container: Query<&mut Style, (With<PhoneUiContainer>, Without<PhoneUiImage>)>,
    mut ui_image: Query<&mut Style, (With<PhoneUiImage>, Without<PhoneUiContainer>)>,
    mut ui_text: Query<&mut Text, With<PhoneUiText>>,
    mut window_resized_events: EventReader<WindowResized>,
) {
    if let Some(event) = window_resized_events.iter().last() {
        let scale = get_display_scale(event.width, event.height);
        // let scale = Vec3::new(event.width, event.height, 1.0);

        for mut style in ui_container.iter_mut() {
            style.size = Size::new(Val::Px(scale.x), Val::Px(scale.y));
        }

        for mut style in ui_image.iter_mut() {
            style.size.width = Val::Px(scale.y * 0.625);
        }

        for mut text in ui_text.iter_mut() {
            for section in text.sections.iter_mut() {
                section.style.font_size = scale.x * 0.035;
            }
        }
    }
}

#[derive(Deref, DerefMut)]
pub(super) struct BackgroundTimer(Timer);

impl Default for BackgroundTimer {
    fn default() -> Self {
        let mut timer = Timer::new(Duration::from_millis(800), false);
        timer.pause();
        Self(timer)
    }
}

pub struct BackgroundTone(pub Handle<AudioInstance>);

pub(super) fn sync_phone_visibility(
    mut cmds: Commands,
    phone_ui_visible: Res<PhoneUiVisible>,
    mut ui_container: Query<&mut Visibility, With<PhoneUiContainer>>,
    //mut windows: ResMut<Windows>,
    //mut fps_controller: Query<&mut FpsController>,
    audio: Res<Audio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    sound_assets: Res<SoundAssets>,
    time: Res<Time>,
    mut background_timer: Local<BackgroundTimer>,
    background_audio_instance: Option<Res<BackgroundTone>>,
) {
    background_timer.tick(time.delta());

    if background_timer.just_finished() {
        let background_audio_instance = audio
            .play(sound_assets.phone_background.clone())
            .looped()
            .with_volume(0.3)
            .handle();

        cmds.insert_resource(BackgroundTone(background_audio_instance));
    }

    if phone_ui_visible.is_changed() {
        if let Ok(mut visibility) = ui_container.get_single_mut() {
            //let primary_win = windows.primary_mut();
            //let mut fps_controller = fps_controller.single_mut();

            if **phone_ui_visible {
                // Unlock
                visibility.is_visible = **phone_ui_visible;
                //fps_controller.enable_input = false;
                //primary_win.set_cursor_visibility(true);
                //primary_win.set_cursor_lock_mode(false);
                background_timer.reset();
                background_timer.unpause();
                audio.play(sound_assets.phone_pickup.clone());
            } else {
                // Lock
                visibility.is_visible = **phone_ui_visible;
                //fps_controller.enable_input = true;
                //primary_win.set_cursor_visibility(false);
                //primary_win.set_cursor_lock_mode(true);
                //primary_win.set_cursor_position(vec2(0.0, 0.0));
                background_timer.reset();
                background_timer.pause();
                audio.play(sound_assets.phone_hangup.clone());
                if let Some(instance) = background_audio_instance {
                    if let Some(instance) = audio_instances.get_mut(&instance.0) {
                        instance.stop(AudioTween::linear(Duration::from_millis(200)));
                    }
                }
            }
        }
    }
}

#[derive(Deref, DerefMut)]
pub(super) struct NumberNotAvailableTimer(Timer);

impl Default for NumberNotAvailableTimer {
    fn default() -> Self {
        let mut timer = Timer::new(Duration::from_secs(3), false);
        timer.pause();
        Self(timer)
    }
}

pub(super) fn number_not_availble(
    mut phone_ui_enabled: ResMut<PhoneUiEnabled>,
    mut phone_text: Query<&mut Text, With<PhoneUiText>>,
    mut phone_submit_events: EventReader<PhoneSubmitEvent>,
    audio: Res<Audio>,
    sound_assets: Res<SoundAssets>,
    time: Res<Time>,
    mut timer: Local<NumberNotAvailableTimer>,
    background_audio_instance: Option<Res<BackgroundTone>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut texts: Query<(&mut Text, &mut NotificationText), Without<PhoneUiText>>,
) {
    timer.tick(time.delta());

    if timer.just_finished() {
        **phone_ui_enabled = true;
        for mut text in &mut phone_text {
            for section in &mut text.sections {
                section.value.clear();
            }
        }
    }

    for ev in phone_submit_events.iter() {
        if ev.number == "5551212" {
            timer.reset();
            timer.unpause();
            audio.play(sound_assets.phone_call.clone());
            for (mut text, mut note) in &mut texts {
                note.0 = 8.0;
                if let Some(section) = text.sections.iter_mut().next() {
                    section.value = String::from(
                        "After seeing\nthe rings\nreversed,\nsomething's\nnot right\nwith the\ngarage.",
                    );
                }
            }
            if let Some(instance_handle) = &background_audio_instance {
                if let Some(instance) = audio_instances.get_mut(&instance_handle.0) {
                    instance.stop(AudioTween::linear(Duration::from_millis(200)));
                }
            }
        } else {
            timer.reset();
            timer.unpause();
            audio.play(sound_assets.phone_number_not_available.clone());
        }
    }
}

pub(super) fn press_phone_keys(
    phone_ui_visible: Res<PhoneUiVisible>,
    mut phone_ui_enabled: ResMut<PhoneUiEnabled>,
    mut phone_keys: Query<(
        Option<&Interaction>,
        Option<ChangeTrackers<Interaction>>,
        &mut UiImage,
        &PhoneKey,
    )>,
    mut phone_text: Query<&mut Text, With<PhoneUiText>>,
    mut phone_submit_events: EventWriter<PhoneSubmitEvent>,
    keys: Res<Input<KeyCode>>,
    image_assets: Res<ImageAssets>,
    audio: Res<Audio>,
    sound_assets: Res<SoundAssets>,
    mut current_pressed_key: Local<Option<PhoneKey>>,
) {
    if **phone_ui_visible && **phone_ui_enabled {
        for (_interaction, _interaction_changes, mut img, phone_key) in &mut phone_keys {
            // Mouse interaction
            // if interaction_changes
            //     .map(|changes| changes.is_changed())
            //     .unwrap_or(false)
            // {
            //     if let Some(interaction) = interaction {
            //         match *interaction {
            //             Interaction::Clicked => {
            //                 for mut text in &mut phone_text {
            //                     if let Some(section) = text.sections.iter_mut().next() {
            //                         section.value.push(phone_key.char());
            //                     }
            //                 }
            //             }
            //             Interaction::Hovered => {
            //                 img.0 = phone_key.image_pressed(&image_assets);
            //             }
            //             Interaction::None => {
            //                 img.0 = phone_key.image(&image_assets);
            //             }
            //         }
            //     }
            // }

            // Keys interaction
            if current_pressed_key.is_none() {
                for key in keys.get_just_pressed() {
                    if phone_key.matches_key(*key) {
                        img.0 = phone_key.image_pressed(&image_assets);
                        for mut text in &mut phone_text {
                            if let Some(section) = text.sections.iter_mut().next() {
                                section.value.push(phone_key.char());

                                if section.value.len() >= 7 {
                                    phone_submit_events.send(PhoneSubmitEvent {
                                        number: section.value.clone(),
                                    });
                                    **phone_ui_enabled = false;
                                    img.0 = phone_key.image(&image_assets);
                                }
                            }
                        }
                        let playback_rate = match phone_key {
                            PhoneKey::Key0 => 0.875,
                            PhoneKey::Key1 => 0.9,
                            PhoneKey::Key2 => 0.925,
                            PhoneKey::Key3 => 0.95,
                            PhoneKey::Key4 => 0.975,
                            PhoneKey::Key5 => 1.0,
                            PhoneKey::Key6 => 1.025,
                            PhoneKey::Key7 => 1.05,
                            PhoneKey::Key8 => 1.075,
                            PhoneKey::Key9 => 1.1,
                            PhoneKey::KeyHash => 1.0,
                            PhoneKey::KeyAsterix => 1.0,
                        };
                        audio
                            .play(sound_assets.phone_key_press.clone())
                            .with_playback_rate(playback_rate)
                            .with_volume(0.1);
                        *current_pressed_key = Some(*phone_key);
                        break;
                    }
                }
            }
            for key in keys.get_just_released() {
                if phone_key.matches_key(*key) {
                    img.0 = phone_key.image(&image_assets);
                    *current_pressed_key = None;
                    break;
                }
            }
        }

        if keys.just_pressed(KeyCode::Back) {
            for mut text in &mut phone_text {
                if let Some(section) = text.sections.iter_mut().next() {
                    section.value.pop();
                }
            }
            audio
                .play(sound_assets.phone_key_press.clone())
                .with_volume(0.1);
        }
    }
}

#[derive(Component)]
pub(super) struct PhoneUiContainer;

#[derive(Component)]
pub(super) struct PhoneUiImage;

#[derive(Component)]
pub(super) struct PhoneUiText;

pub(super) fn setup_phone_ui(
    mut cmds: Commands,
    windows: Res<Windows>,
    font_assets: Res<FontAssets>,
    image_assets: Res<ImageAssets>,
) {
    let window = windows.get_primary().unwrap();

    let scale = get_display_scale(
        window.physical_width() as f32,
        window.physical_height() as f32,
    );
    // let scale = Vec3::new(window.width(), window.height(), 1.0);

    cmds.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(scale.x), Val::Px(scale.y)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
        visibility: Visibility { is_visible: false },
        ..default()
    })
    .insert(PhoneUiContainer)
    .with_children(|parent| {
        parent
            // Container
            .spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(scale.y * 0.625), Val::Percent(100.0)),
                    justify_content: JustifyContent::FlexStart,
                    flex_direction: FlexDirection::ColumnReverse,
                    ..default()
                },
                image: image_assets.phone_base.clone().into(),
                ..default()
            })
            .insert(PhoneUiImage)
            .with_children(|parent| {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::ColumnReverse,
                            align_items: AlignItems::FlexStart,
                            margin: UiRect::new(
                                Val::Auto,
                                Val::Auto,
                                Val::Percent(25.0),
                                Val::Auto,
                            ),
                            size: Size::new(Val::Percent(41.0), Val::Percent(100.0)),
                            position: UiRect::new(
                                Val::Percent(21.0),
                                Val::Undefined,
                                Val::Undefined,
                                Val::Undefined,
                            ),
                            ..default()
                        },
                        color: Color::NONE.into(), // Color::rgba(0.11, 0.2, 0.12, 0.8).into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Entered number box
                        parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(22.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::new(
                                        Val::Undefined,
                                        Val::Undefined,
                                        Val::Undefined,
                                        Val::Percent(30.0),
                                    ),
                                    ..default()
                                },
                                color: Color::NONE.into(), // Color::rgba(0.2, 0.1, 0.12, 0.8).into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                // Entered number
                                parent
                                    .spawn_bundle(TextBundle {
                                        text: Text::from_section(
                                            "",
                                            TextStyle {
                                                font: font_assets.fira_mono_medium.clone(),
                                                font_size: scale.x * 0.035,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        ..default()
                                    })
                                    .insert(PhoneUiText);
                            });

                        let mut spawn_keys = |a: PhoneKey, b: PhoneKey, c: PhoneKey| {
                            // Keys
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                                        justify_content: JustifyContent::FlexEnd,
                                        ..default()
                                    },
                                    color: Color::NONE.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(100.0),
                                                    Val::Percent(100.0),
                                                ),
                                                ..default()
                                            },
                                            color: Color::NONE.into(), // Color::RED.into(),
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            // 1
                                            parent
                                                .spawn_bundle(ButtonBundle {
                                                    image: a.image(&image_assets).into(),
                                                    style: Style {
                                                        size: Size::new(
                                                            Val::Percent(33.0),
                                                            Val::Auto,
                                                        ),
                                                        margin: UiRect::new(
                                                            Val::Px(1.0 * scale.z),
                                                            Val::Px(1.0 * scale.z),
                                                            Val::Px(1.0 * scale.z),
                                                            Val::Px(1.0 * scale.z),
                                                        ),
                                                        ..default()
                                                    },
                                                    ..default()
                                                })
                                                .insert(a);

                                            // 2
                                            parent
                                                .spawn_bundle(ButtonBundle {
                                                    image: b.image(&image_assets).into(),
                                                    style: Style {
                                                        size: Size::new(
                                                            Val::Percent(33.0),
                                                            Val::Auto,
                                                        ),
                                                        margin: UiRect::new(
                                                            Val::Px(1.0 * scale.z),
                                                            Val::Px(1.0 * scale.z),
                                                            Val::Px(1.0 * scale.z),
                                                            Val::Px(1.0 * scale.z),
                                                        ),
                                                        ..default()
                                                    },
                                                    ..default()
                                                })
                                                .insert(b);

                                            // 3
                                            parent
                                                .spawn_bundle(ButtonBundle {
                                                    image: c.image(&image_assets).into(),
                                                    style: Style {
                                                        size: Size::new(
                                                            Val::Percent(33.0),
                                                            Val::Auto,
                                                        ),
                                                        margin: UiRect::new(
                                                            Val::Px(1.0 * scale.z),
                                                            Val::Px(1.0 * scale.z),
                                                            Val::Px(1.0 * scale.z),
                                                            Val::Px(1.0 * scale.z),
                                                        ),
                                                        ..default()
                                                    },
                                                    ..default()
                                                })
                                                .insert(c);
                                        });
                                });
                        };

                        spawn_keys(PhoneKey::Key1, PhoneKey::Key2, PhoneKey::Key3);
                        spawn_keys(PhoneKey::Key4, PhoneKey::Key5, PhoneKey::Key6);
                        spawn_keys(PhoneKey::Key7, PhoneKey::Key8, PhoneKey::Key9);
                        spawn_keys(PhoneKey::KeyAsterix, PhoneKey::Key0, PhoneKey::KeyHash);
                    });
            });
    });
}

pub(super) fn phone_interact_events(
    player_camera: Query<&Transform, With<PlayerCamera>>,
    phones: Query<(Option<&Name>, &Phone)>,
    mut phone_menu_open_events: EventWriter<PhoneMenuOpenEvent>,
    physics_context: Res<RapierContext>,
    mouse_button: Res<Input<MouseButton>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        for transform in player_camera.iter() {
            let max_dist = 2.0;

            let ray = physics_context.cast_ray(
                transform.translation,
                transform.forward(),
                max_dist,
                false,
                QueryFilter::default().exclude_solids(), // Only interact with sensors
            );
            if let Some((entity, _)) = ray {
                if let Ok((name, phone)) = phones.get(entity) {
                    if phone.enabled {
                        let name = name.map(|name| name.to_string());
                        debug!(name = ?name, "Phone menu opened");
                        phone_menu_open_events.send(PhoneMenuOpenEvent {
                            name: name.clone(),
                            entity,
                        });
                    }
                }
            }
        }
    }
}

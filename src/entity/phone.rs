use bevy::{math::vec2, prelude::*};
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    assets::{FontAssets, ImageAssets},
    get_display_scale, spawn_from_scene, PlayerCamera,
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
    pub name: Option<String>,
    pub entity: Entity,
    pub number: u32,
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

enum PhoneKey {
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
}

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

    cmds.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(scale.x), Val::Px(scale.y)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
        ..default()
    })
    .with_children(|parent| {
        parent
            // Container
            .spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(scale.y * 0.625), Val::Px(scale.y)),
                    justify_content: JustifyContent::FlexStart,
                    flex_direction: FlexDirection::ColumnReverse,
                    // padding: UiRect::new(
                    //     Val::Px(6.0 * scale.z),
                    //     Val::Px(6.0 * scale.z),
                    //     Val::Px(6.0 * scale.z),
                    //     Val::Px(6.0 * scale.z),
                    // ),
                    ..default()
                },
                image: image_assets.phone_base.clone().into(),
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            // size: Size::new(Val::Percent(100.0), Val::Auto),
                            // justify_content: JustifyContent::Center,
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
                                parent.spawn_bundle(TextBundle {
                                    text: Text::from_section(
                                        "0422375820",
                                        TextStyle {
                                            font: font_assets.fira_mono_medium.clone(),
                                            font_size: 9.0 * scale.z,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                });
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
                                            parent.spawn_bundle(ImageBundle {
                                                image: a.image(&image_assets).into(),
                                                style: Style {
                                                    size: Size::new(Val::Percent(33.0), Val::Auto),
                                                    margin: UiRect::new(
                                                        Val::Px(1.0 * scale.z),
                                                        Val::Px(1.0 * scale.z),
                                                        Val::Px(1.0 * scale.z),
                                                        Val::Px(1.0 * scale.z),
                                                    ),
                                                    ..default()
                                                },
                                                ..default()
                                            });

                                            // 2
                                            parent.spawn_bundle(ImageBundle {
                                                image: b.image(&image_assets).into(),
                                                style: Style {
                                                    size: Size::new(Val::Percent(33.0), Val::Auto),
                                                    margin: UiRect::new(
                                                        Val::Px(1.0 * scale.z),
                                                        Val::Px(1.0 * scale.z),
                                                        Val::Px(1.0 * scale.z),
                                                        Val::Px(1.0 * scale.z),
                                                    ),
                                                    ..default()
                                                },
                                                ..default()
                                            });

                                            // 3
                                            parent.spawn_bundle(ImageBundle {
                                                image: c.image(&image_assets).into(),
                                                style: Style {
                                                    size: Size::new(Val::Percent(33.0), Val::Auto),
                                                    margin: UiRect::new(
                                                        Val::Px(1.0 * scale.z),
                                                        Val::Px(1.0 * scale.z),
                                                        Val::Px(1.0 * scale.z),
                                                        Val::Px(1.0 * scale.z),
                                                    ),
                                                    ..default()
                                                },
                                                ..default()
                                            });
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
    keys: Res<Input<KeyCode>>,
    mut phone_menu_open_events: EventWriter<PhoneMenuOpenEvent>,
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

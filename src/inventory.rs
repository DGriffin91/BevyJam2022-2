use bevy::{prelude::*, ui::FocusPolicy, window::WindowResized};
use bevy_kira_audio::{prelude::Audio, AudioControl};
use iyes_loopless::prelude::*;

use crate::{
    assets::{GameState, ImageAssets, SoundAssets},
    get_display_scale,
};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Inventory>();
        app.add_enter_system(GameState::RunLevel, create_inventory_toolbar_ui);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::RunLevel)
                .with_system(update_inventory_toolbar_ui)
                .with_system(resize_inventory_toolbar_ui)
                .into(),
        );
    }
}

#[derive(Default, PartialEq, Eq)]
pub struct Inventory {
    pub key: bool,
    pub money: bool,
}

#[derive(Component)]
struct InventoryUiContainer;

#[derive(Component)]
pub struct Icon(&'static str);

fn create_inventory_toolbar_ui(
    mut commands: Commands,
    windows: Res<Windows>,
    image_assets: Res<ImageAssets>,
) {
    let window = windows.get_primary().unwrap();

    let scale = get_display_scale(
        window.physical_width() as f32,
        window.physical_height() as f32,
    );

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(scale.x), Val::Px(scale.y)),
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Percent(0.0),
                    left: Val::Percent(0.0),
                    ..default()
                },
                ..default()
            },
            color: Color::NONE.into(),
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        padding: UiRect::new(
                            Val::Px(20.0),
                            Val::Px(20.0),
                            Val::Px(20.0),
                            Val::Px(20.0),
                        ),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    focus_policy: FocusPolicy::Pass,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Px(42.0), Val::Px(42.0)),
                                margin: UiRect::new(
                                    Val::Px(10.0),
                                    Val::Px(10.0),
                                    Val::Px(10.0),
                                    Val::Px(10.0),
                                ),
                                ..default()
                            },
                            image: image_assets.key.clone().into(),
                            visibility: Visibility { is_visible: false },
                            ..default()
                        })
                        .insert(Icon("key"));
                    parent
                        .spawn_bundle(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Px(42.0), Val::Px(42.0)),
                                margin: UiRect::new(
                                    Val::Px(10.0),
                                    Val::Px(10.0),
                                    Val::Px(10.0),
                                    Val::Px(10.0),
                                ),
                                ..default()
                            },
                            image: image_assets.money.clone().into(),
                            visibility: Visibility { is_visible: false },
                            ..default()
                        })
                        .insert(Icon("money"));
                });
        })
        .insert(InventoryUiContainer);
}

fn resize_inventory_toolbar_ui(
    mut ui: Query<&mut Style, With<InventoryUiContainer>>,
    mut window_resized_events: EventReader<WindowResized>,
) {
    if let Some(event) = window_resized_events.iter().last() {
        for mut style in ui.iter_mut() {
            let scale = get_display_scale(event.width, event.height);
            style.size = Size::new(Val::Px(scale.x), Val::Px(scale.y));
        }
    }
}

fn update_inventory_toolbar_ui(
    inventory: Res<Inventory>,
    mut icons: Query<(&Icon, &mut Visibility)>,
    audio: Res<Audio>,
    sound_assets: Res<SoundAssets>,
) {
    if inventory.is_changed() {
        let mut item_picked_up = false;

        macro_rules! set_icon_visible {
            ($name:literal, $value:expr) => {{
                let mut visibility = icons.iter_mut().find(|icon| icon.0 .0 == $name).unwrap().1;
                let is_visible = visibility.is_visible;
                visibility.is_visible = $value;
                let changed = is_visible != $value;
                if is_visible != $value && $value {
                    item_picked_up = true
                }
                changed
            }};
        }

        set_icon_visible!("key", inventory.key);
        set_icon_visible!("money", inventory.money);

        // TODO only make key sound for keys
        if item_picked_up {
            audio
                .play(sound_assets.keys_pickup.clone())
                .with_volume(0.2);
        }
    }
}

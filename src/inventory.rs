use bevy::{prelude::*, ui::FocusPolicy};
use iyes_loopless::prelude::*;

use crate::assets::{GameState, ImageAssets};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Inventory>();
        app.add_enter_system(GameState::RunLevel, create_inventory_toolbar_ui);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::RunLevel)
                .with_system(insert_item_to_inventory)
                .with_system(update_inventory_toolbar_ui)
                .into(),
        );
    }
}

#[derive(Default, PartialEq, Eq)]
pub struct Inventory {
    key: bool,
}

#[derive(Component)]
struct InventoryUiContainer;

#[derive(Component)]
struct Icon(&'static str);

fn create_inventory_toolbar_ui(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                padding: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Px(20.0), Val::Px(20.0)),
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
        })
        .insert(InventoryUiContainer);
}

fn insert_item_to_inventory(keys: Res<Input<KeyCode>>, mut inventory: ResMut<Inventory>) {
    if keys.just_pressed(KeyCode::E) {
        inventory.key = !inventory.key;
    }
}

fn update_inventory_toolbar_ui(
    inventory: Res<Inventory>,
    mut icons: Query<(&Icon, &mut Visibility)>,
) {
    macro_rules! set_icon_visible {
        ($name:literal, $value:expr) => {
            icons
                .iter_mut()
                .find(|icon| icon.0 .0 == $name)
                .unwrap()
                .1
                .is_visible = $value
        };
    }

    if inventory.is_changed() {
        set_icon_visible!("key", inventory.key);
    }
}

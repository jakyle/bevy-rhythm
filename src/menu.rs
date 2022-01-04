use crate::{consts::*, types::load_config};
use bevy::prelude::*;
use std::fs::read_dir;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu.system()))
            .add_system_set(
                SystemSet::on_update(AppState::Menu)
                    .with_system(button_color_system.system())
                    .with_system(button_press_system.system()),
            )
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(despawn_menu.system()));
    }
}

struct ButtonMaterials {
    none: Handle<ColorMaterial>,
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
    font: Handle<Font>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");

        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

        ButtonMaterials {
            none: materials.add(Color::NONE.into()),
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
            font,
        }
    }
}

struct MenuUI;

enum MenuButton {
    MakeMap,
    PlaySong(String),
}

impl MenuButton {
    fn name(&self) -> String {
        match self {
            MenuButton::MakeMap => "Make map".to_string(),
            MenuButton::PlaySong(song) => format!("Play song: {}", song),
        }
    }
}

fn setup_menu(mut commands: Commands, button_materials: Res<ButtonMaterials>) {
    let mut buttons: Vec<_> = get_songs_iter()
        .map(|name| MenuButton::PlaySong(name))
        .collect();

    buttons.push(MenuButton::MakeMap);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            material: button_materials.none.clone(),
            ..Default::default()
        })
        .insert(MenuUI)
        .with_children(|parent| {
            for button in buttons {
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(350.0), Val::Px(65.0)),
                            margin: Rect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        material: button_materials.normal.clone(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                button.name(),
                                TextStyle {
                                    font: button_materials.font.clone(),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        });
                    })
                    .insert(button);
            }
        });
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn button_color_system(
    button_materials: Res<ButtonMaterials>,
    mut query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => *material = button_materials.pressed.clone(),
            Interaction::Hovered => *material = button_materials.hovered.clone(),
            Interaction::None => *material = button_materials.normal.clone(),
        }
    }
}

fn button_press_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::MakeMap => state
                    .set(AppState::MakeMap)
                    .expect("Couldn't switch state to MakeMap"),
                MenuButton::PlaySong(song) => {
                    let config = load_config(&*format!("{}.toml", song), &asset_server);
                    commands.insert_resource(config);
                    state
                        .set(AppState::Game)
                        .expect("Couldn't switch state to Game")
                }
            }
        }
    }
}

fn get_songs_iter() -> impl Iterator<Item = String> {
    read_dir("assets/songs")
        .unwrap()
        .into_iter()
        .filter_map(|path| path.ok())
        .map(|dir| dir.path())
        .filter(|path| path.as_path().extension().unwrap() == "toml")
        .map(|path| {
            path.as_path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
}

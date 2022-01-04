use arrow::ArrowsPlugin;
use audio::AudioPlugin;
use bevy::{input::system::exit_on_esc_system, prelude::*};
use consts::AppState;
use menu::MenuPlugin;
use score::ScoreResource;
use shaders::ShadersPlugin;
use ui::UIPlugin;

pub mod arrow;
pub mod audio;
pub mod consts;
pub mod menu;
pub mod score;
pub mod shaders;
pub mod types;
pub mod ui;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let config = types::load_config("test.toml", &asset_server);
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .commands()
        .spawn_bundle(UiCameraBundle::default())
        .commands()
        .insert_resource(config);
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Rhythm!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_state(AppState::Menu)
        .init_resource::<ScoreResource>()
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ArrowsPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(ShadersPlugin)
        .add_plugin(MenuPlugin)
        .run();
}

use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy_common_assets::ron::RonAssetPlugin;

use crate::resources::Global;
use crate::states::{GameState, MainState};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetList>()
            .add_plugins(RonAssetPlugin::<GameConfig>::new(&["config.ron"]))
            .insert_resource(Msaa::Off)
            .add_systems(
                Update,
                check_asset_loading.run_if(in_state(MainState::LoadAssets)),
            )
            .add_systems(Update, spawn_level)
            .add_systems(Startup, setup);
    }
}

#[derive(Default, Resource)]
pub struct AssetList(pub Vec<HandleUntyped>);

#[derive(serde::Deserialize, TypeUuid, TypePath, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct GameConfig {
    pub window_title: String,
    window_size: (f32, f32),
    p0_position: (f32, f32),
    p1_position: (f32, f32),
    p2_position: (f32, f32),
    p3_position: (f32, f32),
}

impl GameConfig {
    pub fn p0(&self) -> Vec2 {
        Vec2::from(self.p0_position)
    }
    pub fn p1(&self) -> Vec2 {
        Vec2::from(self.p1_position)
    }
    pub fn p2(&self) -> Vec2 {
        Vec2::from(self.p2_position)
    }
    pub fn p3(&self) -> Vec2 {
        Vec2::from(self.p3_position)
    }
}

#[derive(Resource, Debug)]
pub struct GameConfigHandle(pub Handle<GameConfig>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_list: ResMut<AssetList>,
) {
    let game_config = GameConfigHandle(asset_server.load("game.config.ron"));

    asset_list.0.push(game_config.0.clone_untyped());

    commands.insert_resource(game_config);
}

fn spawn_level(
    config_res: Res<GameConfigHandle>,
    mut game_config: ResMut<Assets<GameConfig>>,

    mut global: ResMut<Global>,
) {
    if let Some(game_config) = game_config.remove(config_res.0.id()) {
        global.game.player_1.draw_pos = game_config.p1();
        global.game.player_2.draw_pos = game_config.p2();
        global.game.player_3.draw_pos = game_config.p3();
        global.game.local_player.draw_pos = game_config.p0();

        info!("Updated {:?}", game_config);
    }
}

pub fn check_asset_loading(
    asset_server: Res<AssetServer>,
    asset_list: Res<AssetList>,
    mut next_state: ResMut<NextState<MainState>>,
) {
    match asset_server.get_group_load_state(asset_list.0.iter().map(|a| a.id())) {
        LoadState::Loading => {
            info!("loading assets...");
            next_state.set(MainState::LoadAssets);
        }
        LoadState::Loaded => {
            info!("loading assets done!");
            next_state.set(MainState::Welcome);
        }
        LoadState::Failed => {
            error!("asset loading error");
        }
        _ => {}
    };
}

use std::collections::HashMap;

use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy_common_assets::ron::RonAssetPlugin;
use naia_bevy_demo_shared::components::card::Card;

use crate::resources::Global;
use crate::states::MainState;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<GameConfig>::new(&["config.ron"]))
            .insert_resource(Msaa::Off)
            .add_systems(Update, spawn_level.run_if(in_state(MainState::Lobby)))
            .add_systems(OnEnter(MainState::LoadAssets), setup);
    }
}

#[derive(Resource, Default)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub noto_font: Handle<Font>,
    pub cards: HashMap<String, Handle<Image>>,
    pub board: Handle<Image>,
    pub back_card: Handle<Image>,
    pub background: Handle<Image>,
    pub play_btn: Handle<Image>,
    pub skip_btn: Handle<Image>,
    pub avatars: HashMap<i32, Handle<Image>>,
}

#[derive(serde::Deserialize, Asset, TypePath, Debug)]
pub struct GameConfig {
    pub window_title: String,
    window_size: (f32, f32),
    pile_position: (f32, f32, f32),
    p0_position: (f32, f32),
    p1_position: (f32, f32),
    p2_position: (f32, f32),
    p3_position: (f32, f32),
}

impl GameConfig {
    pub fn pile_position(&self) -> Vec3 {
        Vec3::from(self.pile_position)
    }
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
    mut next_state: ResMut<NextState<MainState>>,
) {
    let game_config = GameConfigHandle(asset_server.load("game.config.ron"));

    commands.insert_resource(game_config);

    let font = asset_server.load("fonts/font.ttf");

    let noto_font = asset_server.load("fonts/noto.ttf");

    let board = asset_server.load("cards/tables/table_blue.png");

    let back_card =
        asset_server.load("cards/standard/solitaire/individuals/card_back/card_back.png");

    let play_btn = asset_server.load("play.png");

    let skip_btn = asset_server.load("skip.png");

    let background: Handle<Image> = asset_server.load("cards/backgrounds/background_1.png");

    let mut avatars = HashMap::new();

    for i in 0..5 {
        let path = format!("avatars/c{}.png", i);
        let circle_avatar = asset_server.load(path);
        avatars.insert(i, circle_avatar);
    }

    let mut cards = HashMap::new();
    let all_cards: &[Card] = Card::all_cards();

    for card in all_cards {
        let handle = asset_server.load(card.to_path());
        cards.insert(card.name(), handle);
    }

    commands.insert_resource(UiAssets {
        cards,
        font,
        play_btn,
        skip_btn,
        background,
        back_card,
        board,
        noto_font,
        avatars,
    });

    info!("DONE LOAD ASSET!");

    next_state.set(MainState::Welcome);
}

fn spawn_level(
    config_res: Res<GameConfigHandle>,
    mut game_config: ResMut<Assets<GameConfig>>,
    mut global: ResMut<Global>,
) {
    if let Some(game_config) = game_config.remove(config_res.0.id()) {
        info!("Game Config");
        global.game.player_1.draw_pos = game_config.p1();
        global.game.player_2.draw_pos = game_config.p2();
        global.game.player_3.draw_pos = game_config.p3();
        global.game.local_player.draw_pos = game_config.p0();

        global.game.local_player.pile_pos = game_config.pile_position();

        info!("Updated {:?}", game_config);
    }
}

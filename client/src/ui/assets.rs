use bevy::prelude::*;
use naia_bevy_demo_shared::components::card::Card;
use std::collections::HashMap;

use super::UiAssets;

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut asset_list: ResMut<crate::assets::AssetList>,
) {
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
}

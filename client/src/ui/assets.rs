use bevy::prelude::*;
use naia_bevy_demo_shared::components::card::Card;
use std::collections::HashMap;

use super::UiAssets;

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_list: ResMut<crate::assets::AssetList>,
) {
    let font = asset_server.load("fonts/font.ttf");
    asset_list.0.push(font.clone_untyped());

    let noto_font = asset_server.load("fonts/noto.ttf");
    asset_list.0.push(noto_font.clone_untyped());

    let board = asset_server.load("cards/tables/table_blue.png");
    asset_list.0.push(board.clone_untyped());

    let back_card =
        asset_server.load("cards/standard/solitaire/individuals/card_back/card_back.png");

    asset_list.0.push(back_card.clone_untyped());

    let play_btn = asset_server.load("play.png");

    asset_list.0.push(play_btn.clone_untyped());

    let skip_btn = asset_server.load("skip.png");

    asset_list.0.push(skip_btn.clone_untyped());

    let background: Handle<Image> = asset_server.load("cards/backgrounds/background_1.png");
    asset_list.0.push(background.clone_untyped());

    let mut avatars = HashMap::new();

    for i in 0..5 {
        let path = format!("avatars/c{}.png", i);
        let circle_avatar = asset_server.load(path);
        asset_list.0.push(circle_avatar.clone_untyped());
        avatars.insert(i, circle_avatar);
    }

    let mut cards = HashMap::new();
    let all_cards: &[Card] = Card::all_cards();

    for card in all_cards {
        let handle = asset_server.load(card.to_path());
        asset_list.0.push(handle.clone_untyped());
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

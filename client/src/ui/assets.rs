use bevy::prelude::*;
use naia_bevy_demo_shared::components::card::Card;
use std::collections::HashMap;

use super::UiAssets;

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_list: ResMut<crate::assets::AssetList>,
) {
    let font = asset_server.load("fonts/FiraCode-Bold.otf");
    asset_list.0.push(font.clone_untyped());

    let board = asset_server.load("cards/tables/table_blue.png");
    asset_list.0.push(board.clone_untyped());

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
        board,
        avatars,
    });
}

use std::collections::HashMap;

use bevy::prelude::*;

use crate::states::MainState;

pub mod assets;
mod play_btn;
pub mod player;
pub mod table;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ReloadUiEvent>()
            .add_event::<ReloadBar>()
            .add_event::<DrawPlayer>()
            .add_event::<UpdateCard>()
            .add_startup_system(assets::load_assets)
            .add_system(table::spawn_table.in_schedule(OnEnter(MainState::Game)))
            .add_system(table::draw_table.run_if(in_state(MainState::Game)))
            // .add_system(hand::draw_player.run_if(on_event::<ReloadUiEvent>()))
            // .add_system(hand::draw_player.in_schedule(OnEnter(GameState::PlayerInput)))
            // .add_system(hand::card_click.in_set(OnUpdate(MainState::Game)))
            .add_system(play_btn::spawn_play_btn.in_schedule(OnEnter(MainState::Lobby)))
            .add_system(
                player::card_click
                    .run_if(in_state(MainState::Game))
                    .before(player::draw_player),
            )
            // .add_system(player::draw_player.run_if(on_event::<DrawPlayer>()))
            .add_system(player::draw_player.run_if(in_state(MainState::Game)))
            .add_system(play_btn::spawn_start_btn.run_if(on_event::<ReloadBar>()))
            .add_system(play_btn::player_btn_click.run_if(in_state(MainState::Lobby)))
            .add_system(play_btn::player_btn_click.run_if(in_state(MainState::Game)));
        // .add_system(play_btn::update_turn_timer.run_if(in_state(MainState::Game)));
    }
}

pub struct ReloadUiEvent;
pub struct ReloadBar;
pub struct DrawPlayer;
pub struct UpdateCard;

#[derive(Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub cards: HashMap<String, Handle<Image>>,
    pub board: Handle<Image>,
}

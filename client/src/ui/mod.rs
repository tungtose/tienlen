use std::collections::HashMap;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;

use crate::{game::LocalStartGame, states::MainState};

pub mod assets;
mod play_btn;
pub mod player;
pub mod playerui;
pub mod table;
pub mod timer;

const FIXED_TIMESTEP: f32 = 0.5;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ReloadUiEvent>()
            .add_event::<ReloadBar>()
            .add_event::<DrawPlayer>()
            .add_event::<UpdateCard>()
            .add_event::<DrawStatus>()
            .add_event::<UpdateScoreUI>()
            .add_plugin(ShapePlugin)
            .add_startup_system(assets::load_assets)
            .add_system(playerui::spawn_playerui.run_if(on_event::<LocalStartGame>()))
            .add_system(playerui::circle_cooldown_update.run_if(in_state(MainState::Game)))
            .add_system(playerui::update_score.run_if(on_event::<UpdateScoreUI>()))
            .add_system(playerui::animatetext_update.in_schedule(CoreSchedule::FixedUpdate))
            .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
            // .add_system(playerui::animatetext_update.run_if(in_state(MainState::Game)))
            .add_system(table::spawn_table.in_schedule(OnEnter(MainState::Game)))
            .add_system(table::draw_table.run_if(in_state(MainState::Game)))
            .add_system(table::draw_status.run_if(on_event::<DrawStatus>()))
            .add_system(table::delete_status.run_if(in_state(MainState::Game)))
            .add_system(play_btn::spawn_play_btn.run_if(in_state(MainState::Game)))
            .add_system(
                player::card_click
                    .run_if(in_state(MainState::Game))
                    .before(player::draw_player),
            )
            .add_system(timer::draw_counter.run_if(in_state(MainState::Game)))
            .add_system(player::draw_player.run_if(on_event::<DrawPlayer>()))
            .add_system(play_btn::spawn_start_btn.run_if(on_event::<ReloadBar>()))
            .add_system(play_btn::player_btn_click.run_if(in_state(MainState::Lobby)))
            .add_system(play_btn::player_btn_click.run_if(in_state(MainState::Game)));
    }
}

pub struct ReloadUiEvent;
pub struct ReloadBar;

#[derive(Default)]
pub struct UpdateScoreUI;

#[derive(Default)]
pub struct DrawPlayer;
pub struct DrawStatus(pub String);
pub struct UpdateCard;

#[derive(Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub cards: HashMap<String, Handle<Image>>,
    pub board: Handle<Image>,
    pub avatars: HashMap<i32, Handle<Image>>,
    // pub avatars: Handle<Image>,
}

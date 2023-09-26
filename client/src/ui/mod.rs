use std::collections::HashMap;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;

use crate::states::MainState;

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
            .add_event::<SpawnLocalPlayerEvent>()
            .add_event::<ReloadBar>()
            .add_event::<PlayerMessageEvent>()
            .add_event::<DrawPlayer>()
            .add_event::<UpdateCard>()
            .add_event::<DrawStatus>()
            .add_event::<UpdateScoreUI>()
            .add_event::<NewPlayerJoin>()
            .add_plugins(ShapePlugin)
            .add_systems(Startup, assets::load_assets)
            .add_systems(
                Update,
                playerui::draw_player_ui.run_if(in_state(MainState::Lobby)),
            )
            .add_systems(
                Update,
                playerui::circle_cooldown_update.run_if(in_state(MainState::Game)),
            )
            .add_systems(
                Update,
                playerui::update_score.run_if(in_state(MainState::Game)),
            )
            .add_systems(
                Update,
                playerui::update_player_message.run_if(on_event::<PlayerMessageEvent>()),
            )
            .add_systems(
                Update,
                playerui::clean_player_message.run_if(in_state(MainState::Game)),
            )
            .add_systems(
                FixedUpdate,
                playerui::animatetext_update.run_if(in_state(MainState::Game)),
            )
            .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
            .add_systems(OnEnter(MainState::Lobby), table::spawn_table)
            .add_systems(
                Update,
                table::draw_table
                    .run_if(in_state(MainState::Lobby).or_else(in_state(MainState::Game))),
            )
            // .add_system(table::draw_table.run_if(in_state(MainState::Game)))
            .add_systems(Update, table::draw_status.run_if(on_event::<DrawStatus>()))
            .add_systems(
                Update,
                table::delete_status.run_if(in_state(MainState::Game)),
            )
            .add_systems(
                Update,
                play_btn::spawn_play_btn.run_if(in_state(MainState::Game)),
            )
            .add_systems(
                Update,
                player::card_click
                    .run_if(in_state(MainState::Game))
                    .before(player::draw_player),
            )
            .add_systems(OnEnter(MainState::Game), timer::init_counter)
            .add_systems(
                Update,
                timer::update_counter.run_if(in_state(MainState::Game)),
            )
            .add_systems(
                Update,
                player::draw_player.run_if(in_state(MainState::Game)),
            )
            .add_systems(
                Update,
                play_btn::spawn_start_btn.run_if(on_event::<ReloadBar>()),
            )
            .add_systems(
                Update,
                play_btn::player_btn_click.run_if(in_state(MainState::Lobby)),
            )
            .add_systems(
                Update,
                play_btn::player_btn_click.run_if(in_state(MainState::Game)),
            )
            .add_systems(OnEnter(MainState::Game), play_btn::hide_start_btn);
    }
}

#[derive(Default, Event)]
pub struct ReloadUiEvent;
#[derive(Default, Event)]
pub struct ReloadBar;

#[derive(Default, Event)]
pub struct UpdateScoreUI;

#[derive(Default, Event)]
pub struct NewPlayerJoin;

#[derive(Default, Event)]
pub struct SpawnLocalPlayerEvent;

#[derive(Default, Event)]
pub struct PlayerMessageEvent(pub usize, pub String);

#[derive(Default, Event)]
pub struct DrawPlayer;
#[derive(Default, Event)]
pub struct DrawStatus(pub String);
#[derive(Default, Event)]
pub struct UpdateCard;

#[derive(Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub cards: HashMap<String, Handle<Image>>,
    pub board: Handle<Image>,
    pub avatars: HashMap<i32, Handle<Image>>,
}

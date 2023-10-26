mod cards;
mod controller;
mod player_ui;
mod status;
mod table;
use std::collections::BTreeMap;

use bevy::prelude::*;
use naia_bevy_client::{events::MessageEvents, Client};
use naia_bevy_demo_shared::{
    channels::{GameSystemChannel, PlayerActionChannel},
    components::{card::Card, hand::Hand, Player},
    messages::{AcceptStartGame, EndMatch, SkipTurn},
};

use crate::{components::LocalPlayer, resources::Global, states::MainState};

use self::{
    cards::CardPlugin, controller::ControllerPlugin, player_ui::PlayerUiPlugin, status::DrawStatus,
    table::TablePlugin,
};
use self::{controller::SkipTurnEvent, status::StatusPlugin};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LocalStartGame>()
            .add_event::<UpdatePlayerCards>()
            .add_event::<SelectCardEvent>()
            .add_plugins(StatusPlugin)
            .add_plugins(CardPlugin)
            .add_plugins(ControllerPlugin)
            .add_plugins(PlayerUiPlugin)
            .add_plugins(TablePlugin)
            .add_systems(Startup, local_init)
            // .add_systems(Update, spawn_player.run_if(on_event::<LocalStartGame>()))
            .add_systems(Update, wait_to_ingame.run_if(in_state(MainState::Wait)))
            // .add_systems(Update, play_card.run_if(on_event::<PlayerEvent>()))
            .add_systems(Update, skip_turn.run_if(on_event::<SkipTurnEvent>()));
    }
}

#[derive(Event)]
pub struct LocalStartGame(pub String);
#[derive(Event)]
pub struct UpdatePlayerCards;

#[derive(Event)]
pub struct SelectCardEvent(pub usize);

#[derive(Component)]
pub struct ActiveCard(pub bool);

#[derive(Component, Default)]
pub struct ActiveCards(BTreeMap<usize, Card>);

#[derive(Component, Default)]
pub struct LocalPlayerCards(pub BTreeMap<usize, Card>);

fn local_init(mut commands: Commands) {
    commands.spawn(ActiveCards::default());
    commands.spawn(LocalPlayerCards::default());
}

pub fn skip_turn(mut client: Client) {
    info!("skip turn!!!");
    client.send_message::<PlayerActionChannel, SkipTurn>(&SkipTurn::default());
}

pub fn wait_to_ingame(
    mut event_reader: EventReader<MessageEvents>,
    mut next_state: ResMut<NextState<MainState>>,
) {
    for event in event_reader.iter() {
        for _ in event.read::<GameSystemChannel, AcceptStartGame>() {
            info!("Switching to InGame");
            next_state.set(MainState::Game);
        }
    }
}

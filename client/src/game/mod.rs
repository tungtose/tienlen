mod cards;
mod status;
mod table;
use std::collections::BTreeMap;

use bevy::prelude::*;
use naia_bevy_client::Client;
use naia_bevy_demo_shared::{
    channels::PlayerActionChannel,
    components::{card::Card, hand::Hand, Player},
    messages::{PlayCard, SkipTurn},
};

use crate::{components::LocalPlayer, resources::Global, states::MainState, ui::DrawStatus};

use self::{cards::CardPlugin, table::TablePlugin};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LocalStartGame>()
            .add_event::<PlayerEvent>()
            .add_event::<SkipTurnEvent>()
            .add_event::<UpdatePlayerCards>()
            .add_event::<SelectCardEvent>()
            .add_plugins(CardPlugin)
            .add_plugins(TablePlugin)
            .add_systems(Startup, local_init)
            .add_systems(Update, spawn_player.run_if(on_event::<LocalStartGame>()))
            .add_systems(
                Update,
                update_player_cards.run_if(on_event::<UpdatePlayerCards>()),
            )
            .add_systems(Update, play_card.run_if(on_event::<PlayerEvent>()))
            .add_systems(Update, skip_turn.run_if(on_event::<SkipTurnEvent>()))
            .add_systems(Update, select_card.run_if(on_event::<SelectCardEvent>()));
    }
}

#[derive(Event)]
pub struct LocalStartGame(pub String);
#[derive(Event)]
pub struct UpdatePlayerCards;

#[derive(Event)]
pub struct SelectCardEvent(pub usize);

#[derive(Event)]
pub struct PlayerEvent;
#[derive(Event)]
pub struct SkipTurnEvent;

#[derive(Component)]
pub struct ActiveCard(pub bool);

#[derive(Component, Default)]
pub struct ActiveCards(BTreeMap<usize, Card>);

#[derive(Component, Default)]
pub struct LocalPlayerCards(pub BTreeMap<usize, Card>);

impl ActiveCards {
    pub fn make_active(&mut self, key: &usize, card: &Card) {
        if self.0.contains_key(key) {
            self.0.remove(key);
        } else {
            self.0.insert(*key, *card);
        }
    }

    pub fn keys(&self) -> Vec<&usize> {
        self.0.keys().clone().collect::<Vec<&usize>>()
    }

    pub fn to_vec(&self) -> Vec<Card> {
        {
            let mut cards_vec = vec![];
            for card in self.0.values() {
                cards_vec.push(*card);
            }
            cards_vec
        }
    }

    pub fn to_string(&self) -> Result<String, &'static str> {
        if self.is_empty() {
            return Err("Not have any active card");
        }

        Ok(self
            .to_vec()
            .iter()
            .map(|card| card.to_str())
            .collect::<Vec<String>>()
            .join(","))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

fn local_init(mut commands: Commands) {
    commands.spawn(ActiveCards::default());
    commands.spawn(LocalPlayerCards::default());
}

pub fn select_card(
    mut active_cards_q: Query<&mut ActiveCards>,
    global: Res<Global>,
    mut select_card_ev: EventReader<SelectCardEvent>,
) {
    for event in select_card_ev.iter() {
        // BUG: there is a panic here!
        info!("try select card: {:?}", &event.0);
        let card = global.player_cards.get(&event.0).unwrap();
        active_cards_q
            .get_single_mut()
            .unwrap()
            .make_active(&event.0, card);
    }
}

pub fn skip_turn(mut client: Client) {
    info!("skip turn!!!");
    client.send_message::<PlayerActionChannel, SkipTurn>(&SkipTurn::default());
}

pub fn play_card(
    mut active_cards_q: Query<&mut ActiveCards>,
    mut client: Client,
    mut draw_status_ev: EventWriter<DrawStatus>,
) {
    info!("Play Card!");
    let active_cards_map = active_cards_q.get_single_mut().unwrap();

    let Ok(cards) = active_cards_map.to_string() else {
        draw_status_ev.send(DrawStatus("Please select any cards".to_string()));
        return;
    };

    let hand = Hand::from_str(&cards);

    if !hand.check_combination() {
        draw_status_ev.send(DrawStatus(
            "Your hand is not in any combination!".to_string(),
        ));
        return;
    }
    client.send_message::<PlayerActionChannel, PlayCard>(&PlayCard(cards));

    info!("Sended Cards");
}

pub fn update_player_cards(
    player_q: Query<&Player, With<LocalPlayer>>,
    mut global: ResMut<Global>,
    mut active_cards_q: Query<&mut ActiveCards>,
) {
    let Ok(player) = player_q.get_single() else {
        return;
    };

    let hand_str = player.cards.clone();

    let hand = Hand::from(hand_str);

    global.player_cards.clear();

    if hand.is_empty() {
        return;
    }

    for card in hand.cards.as_slice() {
        global.player_cards.insert(card.ordinal(), *card);
    }

    let mut active_cards_map = active_cards_q.get_single_mut().unwrap();
    active_cards_map.keys().iter().for_each(|key| {
        global.player_cards.remove(key);
    });
    active_cards_map.clear();
}

pub fn spawn_player(
    mut next_state: ResMut<NextState<MainState>>,
    player_q: Query<&Player, With<LocalPlayer>>,
    mut global: ResMut<Global>,
) {
    let Ok(player) = player_q
        .get_single()
         else {
        return;
    };

    let hand_str = player.cards.clone();
    let hand = Hand::from(hand_str);

    for card in hand.cards.as_slice() {
        global.player_cards.insert(card.ordinal(), *card);
    }

    next_state.set(MainState::Game);
}
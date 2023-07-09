use std::collections::BTreeMap;

use bevy::prelude::*;
use naia_bevy_client::Client;
use naia_bevy_demo_shared::{
    channels::PlayerActionChannel,
    components::{card::Card, hand::Hand, server_hand::ServerHand, Player, Table},
    messages::PlayCard,
};

use crate::{
    components::LocalPlayer,
    resources::Global,
    states::MainState,
    ui::{table::Status, DrawPlayer, DrawStatus},
};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LocalStartGame>()
            .add_event::<PlayerEvent>()
            .add_event::<SelectCardEvent>()
            .add_startup_system(local_init)
            .add_system(spawn_player.run_if(on_event::<LocalStartGame>()))
            .add_system(play_card.run_if(on_event::<PlayerEvent>()))
            .add_system(select_card.run_if(on_event::<SelectCardEvent>()));
    }
}

pub struct LocalStartGame;

pub struct SelectCardEvent(pub usize);

pub struct PlayerEvent;

#[derive(Component)]
pub struct ActiveCard(pub bool);

#[derive(Component)]
pub struct ActiveCards(BTreeMap<usize, Card>);

impl Default for ActiveCards {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl ActiveCards {
    pub fn is_active(&self, key: &usize) -> bool {
        self.0.contains_key(key)
    }
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

    pub fn to_vec(&mut self) -> Vec<Card> {
        let cards = {
            let mut cards_vec = vec![];
            for card in self.0.values() {
                cards_vec.push(*card);
            }
            cards_vec
        };
        cards
    }

    pub fn to_string(&mut self) -> Result<String, &'static str> {
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
}

pub fn select_card(
    mut active_cards_q: Query<&mut ActiveCards>,
    global: Res<Global>,
    mut draw_player_ev: EventWriter<DrawPlayer>,
    mut select_card_ev: EventReader<SelectCardEvent>,
) {
    for event in select_card_ev.iter() {
        let card = global.player_cards.get(&event.0).unwrap();
        active_cards_q
            .get_single_mut()
            .unwrap()
            .make_active(&event.0, card);
        draw_player_ev.send(DrawPlayer);
    }
}

pub fn play_card(
    mut active_cards_q: Query<&mut ActiveCards>,
    mut client: Client,
    // mut draw_player_ev: EventWriter<DrawPlayer>,
    // mut global: ResMut<Global>,
    mut draw_status_ev: EventWriter<DrawStatus>,
    mut status_q: Query<&mut Status>,
    player_q: Query<&Player, With<LocalPlayer>>,
) {
    // Check if is valid to play
    let Ok(player) = player_q.get_single() else {
        return;
    };

    if !*player.active {
        // Not your turn now
        return;
    }

    info!("Play Card!");
    let mut active_cards_map = active_cards_q.get_single_mut().unwrap();

    let Ok(cards) = active_cards_map.to_string() else {
        let mut status = status_q.get_single_mut().unwrap();

        status.0 = "Please select any cards".to_string();
        draw_status_ev.send(DrawStatus);
        
        return
    };

    let hand = Hand::from_str(&cards);

    if !hand.check_combination() {
        info!("hand {} not in thirteen combination", hand);
        return;
    }

    // active_cards_map.keys().iter().for_each(|key| {
    //     global.player_cards.remove(key);
    // });
    //
    // active_cards_map.clear();
    client.send_message::<PlayerActionChannel, PlayCard>(&PlayCard(cards));

    // draw_player_ev.send(DrawPlayer);
    info!("Sended Cards");
}

pub fn spawn_player(
    mut next_state: ResMut<NextState<MainState>>,
    hand_q: Query<&ServerHand, With<LocalPlayer>>,
    mut global: ResMut<Global>,
    mut draw_player_ev: EventWriter<DrawPlayer>,
) {
    let Ok(server_hand) = hand_q
        .get_single()
         else {
        return;
    };

    let hand_str = server_hand
        .cards
        .split(",")
        .map(|c| c.to_string())
        .collect::<Vec<String>>();

    let sl: Vec<&str> = hand_str.iter().map(|str| str.as_str()).collect();

    for card_str in sl {
        let card_rs = Card::from_str(card_str);

        if let Ok(card) = card_rs {
            // commands.spawn(card).id();
            global.player_cards.insert(card.ordinal(), card);
        } else {
            info!("SPAWN CARD ERROR: {}", card_str);
        }
    }
    next_state.set(MainState::Game);
    draw_player_ev.send(DrawPlayer);
}

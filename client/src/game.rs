use bevy::prelude::*;
use naia_bevy_demo_shared::components::{card::Card, server_hand::ServerHand};

use crate::{components::LocalPlayer, ui::DrawPlayer};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LocalStartGame>()
            .add_system(spawn_player.run_if(on_event::<LocalStartGame>()));
    }
}

pub struct LocalStartGame;
#[derive(Component)]
pub struct ActiveCard(pub bool);

pub fn spawn_player(
    mut commands: Commands,
    hand_q: Query<&ServerHand, With<LocalPlayer>>,
    player_q: Query<Entity, With<LocalPlayer>>,
    mut draw_player_ev: EventWriter<DrawPlayer>,
) {
    let hand_str = hand_q
        .get_single()
        .unwrap()
        .cards
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>();

    let sl: Vec<&str> = hand_str.iter().map(|str| str.as_str()).collect();

    let player_entity = player_q.get_single().unwrap();

    info!("cards: {:?}", sl);

    for card_str in sl {
        let card_rs = Card::from_str(card_str);

        if let Ok(card) = card_rs {
            info!("Inserting cards: {}", card.to_str());
            let card_entity = commands.spawn((card, ActiveCard(false))).id();
            commands.entity(player_entity).add_child(card_entity);
        } else {
            info!("SPAWN CARD ERROR: {}", card_str);
        }
    }

    draw_player_ev.send(DrawPlayer);
}

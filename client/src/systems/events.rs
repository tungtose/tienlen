use std::default::Default;

use bevy::prelude::{info, Commands, EventReader, EventWriter, NextState, Query, ResMut};

use naia_bevy_client::{
    events::{
        ClientTickEvent, ConnectEvent, DespawnEntityEvent, DisconnectEvent, MessageEvents,
        RejectEvent, SpawnEntityEvent,
    },
    Client,
};

use naia_bevy_demo_shared::{
    channels::{
        EntityAssignmentChannel, GameSystemChannel, PlayerActionChannel, PlayerCommandChannel,
    },
    components::player::Player,
    messages::{
        AcceptStartGame, EntityAssignment, ErrorCode, GameError, KeyCommand, NewMatch, NewPlayer,
        PlayerMessage, PlayerReady, StartGame, UpdateScore, UpdateTurn,
    },
};

use crate::{
    components::LocalPlayer,
    game::{LocalStartGame, UpdatePlayerCards},
    resources::Global,
    states::MainState,
    ui::{DrawStatus, NewPlayerJoin, PlayerMessageEvent, UpdateScoreUI},
};

pub fn connect_events(
    mut client: Client,
    global: ResMut<Global>,
    mut next_state: ResMut<NextState<MainState>>,
    mut event_reader: EventReader<ConnectEvent>,
) {
    for _ in event_reader.iter() {
        let Ok(server_address) = client.server_address() else {
            panic!("Not found server address!");
        };
        info!("Client connected to: {}", server_address);

        client
            .send_message::<PlayerActionChannel, NewPlayer>(&NewPlayer(global.player_name.clone()));

        next_state.set(MainState::Lobby);
    }
}

pub fn reject_events(mut event_reader: EventReader<RejectEvent>) {
    for _ in event_reader.iter() {
        info!("Client rejected from connecting to Server");
    }
}

pub fn disconnect_events(mut event_reader: EventReader<DisconnectEvent>) {
    for _ in event_reader.iter() {
        info!("Client disconnected from Server");
    }
}

#[allow(clippy::too_many_arguments)]
pub fn message_events(
    mut commands: Commands,
    mut client: Client,
    mut global: ResMut<Global>,
    mut event_reader: EventReader<MessageEvents>,
    player_query: Query<&Player>,
    mut start_game_ev: EventWriter<LocalStartGame>,
    mut draw_status_ev: EventWriter<DrawStatus>,
    mut update_player_cards_ev: EventWriter<UpdatePlayerCards>,
    mut update_score_ev: EventWriter<UpdateScoreUI>,
    mut new_player_join_ev: EventWriter<NewPlayerJoin>,
    mut player_message_ev: EventWriter<PlayerMessageEvent>,
) {
    for events in event_reader.iter() {
        for message in events.read::<GameSystemChannel, PlayerMessage>() {
            let event = PlayerMessageEvent(message.0, message.1.to_string());
            player_message_ev.send(event);
        }

        for message in events.read::<GameSystemChannel, AcceptStartGame>() {
            global.game.active_player_pos = 0;
            start_game_ev.send(LocalStartGame(message.cards));
        }

        for _ in events.read::<GameSystemChannel, NewPlayer>() {
            new_player_join_ev.send_default();
        }

        for _ in events.read::<GameSystemChannel, UpdateScore>() {
            update_score_ev.send_default();
        }

        for error_code in events.read::<GameSystemChannel, ErrorCode>() {
            let game_error = GameError::from(error_code);
            match game_error {
                GameError::InvalidCards => {
                    draw_status_ev.send(DrawStatus("Your cards are week!".to_string()));
                }
                GameError::WrongCombination => {
                    draw_status_ev.send(DrawStatus(
                        "Your cards are not the same combination".to_string(),
                    ));
                }
                GameError::CanNotSkipTurn => {
                    draw_status_ev.send(DrawStatus(
                        "You can not skip turn, you can play any card now".to_string(),
                    ));
                }
                GameError::WrongTurn => {
                    draw_status_ev.send(DrawStatus(
                        "Not your turn now! Game bug probably".to_string(),
                    ));
                }
                GameError::UnknownError => {
                    draw_status_ev.send(DrawStatus("Unexpected error happend".to_string()));
                }
            }
        }

        for _ in events.read::<GameSystemChannel, NewMatch>() {
            update_player_cards_ev.send(UpdatePlayerCards)
        }

        for update_turn in events.read::<GameSystemChannel, UpdateTurn>() {
            let active_player_pos = update_turn.0 as i32;
            global.game.active_player_pos = active_player_pos;
        }

        for message in events.read::<EntityAssignmentChannel, EntityAssignment>() {
            let assign = message.assign;

            let entity = message.entity.get(&client).unwrap();
            if assign {
                info!("gave ownership of entity");

                match player_query.get(entity) {
                    Ok(_) => {
                        info!("CONNECTED!!!");
                        global.player_entity = Some(entity);
                        commands.entity(entity).insert(LocalPlayer);

                        client.send_message::<PlayerActionChannel, PlayerReady>(
                            &PlayerReady::default(),
                        );
                    }
                    Err(err) => info!("Gave Ownership Error: {}", err),
                }
            }
        }
    }
}

pub fn spawn_entity_events(mut event_reader: EventReader<SpawnEntityEvent>) {
    for SpawnEntityEvent(_entity) in event_reader.iter() {
        info!("spawned entity");
    }
}

pub fn despawn_entity_events(mut event_reader: EventReader<DespawnEntityEvent>) {
    for DespawnEntityEvent(_entity) in event_reader.iter() {
        info!("despawned entity");
    }
}

pub fn tick_events(
    mut client: Client,
    mut global: ResMut<Global>,
    mut tick_reader: EventReader<ClientTickEvent>,
) {
    let Some(command) = global.queued_command.take() else {
        return;
    };

    for ClientTickEvent(client_tick) in tick_reader.iter() {
        if !global.command_history.can_insert(client_tick) {
            // History is full
            continue;
        }

        // Record command
        global.command_history.insert(*client_tick, command.clone());

        // Send command
        client.send_tick_buffer_message::<PlayerCommandChannel, KeyCommand>(client_tick, &command);
    }
}

use std::default::Default;

use bevy::{
    prelude::{
        info, Color as BevyColor, Commands, EventReader, EventWriter, NextState, Query, Res,
        ResMut, Sprite, SpriteBundle, Transform, Vec2,
    },
    sprite::MaterialMesh2dBundle,
};

use naia_bevy_client::{
    events::{
        ClientTickEvent, ConnectEvent, DespawnEntityEvent, DisconnectEvent, InsertComponentEvents,
        MessageEvents, RejectEvent, RemoveComponentEvents, SpawnEntityEvent, UpdateComponentEvents,
    },
    sequence_greater_than, Client, Replicate, Tick,
};

use naia_bevy_demo_shared::{
    behavior as shared_behavior,
    channels::{
        EntityAssignmentChannel, GameSystemChannel, PlayerActionChannel, PlayerCommandChannel,
    },
    components::{player::Player, Color, ColorValue, Position, Shape, ShapeValue},
    messages::{
        Counter, EntityAssignment, ErrorCode, GameError, KeyCommand, NewMatch, NewPlayer, PlayCard,
        PlayerMessage, StartGame, UpdateScore, UpdateTurn,
    },
};

use crate::{
    components::{Confirmed, Interp, LocalPlayer},
    game::{LocalStartGame, UpdatePlayerCards},
    resources::Global,
    states::MainState,
    ui::{
        DrawStatus, NewPlayerJoin, PlayerMessageEvent, ReloadBar, SpawnLocalPlayerEvent,
        UpdateScoreUI,
    },
};

const SQUARE_SIZE: f32 = 32.0;

pub fn connect_events(
    // mut commands: Commands,
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
    client: Client,
    mut global: ResMut<Global>,
    mut event_reader: EventReader<MessageEvents>,
    player_query: Query<&Player>,
    mut bar_ev: EventWriter<ReloadBar>,
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

        for _ in events.read::<GameSystemChannel, StartGame>() {
            global.active_player_pos = 0;
            start_game_ev.send(LocalStartGame);
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
                GameError::WrongTurn => todo!(),
                GameError::UnknownError => todo!(),
            }
        }

        for _ in events.read::<GameSystemChannel, NewMatch>() {
            update_player_cards_ev.send(UpdatePlayerCards)
        }

        for _ in events.read::<GameSystemChannel, PlayCard>() {
            update_player_cards_ev.send(UpdatePlayerCards)
        }

        for update_turn in events.read::<GameSystemChannel, UpdateTurn>() {
            let active_player_pos = update_turn.0 as i32;
            global.active_player_pos = active_player_pos;
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
                        bar_ev.send(ReloadBar);
                    }
                    Err(err) => info!("Error: {}", err),
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

pub fn insert_component_events(
    mut commands: Commands,
    mut event_reader: EventReader<InsertComponentEvents>,
    sprite_query: Query<(&Shape, &Color)>,
    position_query: Query<&Position>,
) {
    for events in event_reader.iter() {
        for entity in events.read::<Color>() {
            // When we receive a replicated Color component for a given Entity,
            // use that value to also insert a local-only SpriteBundle component into this entity
            info!("add Color Component to entity");

            if let Ok((shape, color)) = sprite_query.get(entity) {}
        }
        for entity in events.read::<Position>() {
            info!("add Position Component to entity");
            if let Ok(position) = position_query.get(entity) {
                // initialize interpolation
                commands
                    .entity(entity)
                    .insert(Interp::new(*position.x, *position.y));
            }
        }
    }
}

pub fn update_component_events(
    mut global: ResMut<Global>,
    mut event_reader: EventReader<UpdateComponentEvents>,
    mut position_query: Query<&mut Position>,
) {
    // When we receive a new Position update for the Player's Entity,
    // we must ensure the Client-side Prediction also remains in-sync
    // So we roll the Prediction back to the authoritative Server state
    // and then execute all Player Commands since that tick, using the CommandHistory helper struct
    // if let Some(owned_entity) = &global.owned_entity {
    //     let mut latest_tick: Option<Tick> = None;
    //     let server_entity = owned_entity.confirmed;
    //     let client_entity = owned_entity.predicted;
    //
    //     for events in event_reader.iter() {
    //         // Update square position
    //         for (server_tick, updated_entity) in events.read::<Position>() {
    //             // If entity is owned
    //             if updated_entity == server_entity {
    //                 if let Some(last_tick) = &mut latest_tick {
    //                     if sequence_greater_than(server_tick, *last_tick) {
    //                         *last_tick = server_tick;
    //                     }
    //                 } else {
    //                     latest_tick = Some(server_tick);
    //                 }
    //             }
    //         }
    //     }
    //
    //     if let Some(server_tick) = latest_tick {
    //         if let Ok([server_position, mut client_position]) =
    //             position_query.get_many_mut([server_entity, client_entity])
    //         {
    //             // Set to authoritative state
    //             client_position.mirror(&*server_position);
    //
    //             // Replay all stored commands
    //
    //             // TODO: why is it necessary to subtract 1 Tick here?
    //             // it's not like this in the Macroquad demo
    //             let modified_server_tick = server_tick.wrapping_sub(1);
    //
    //             let replay_commands = global.command_history.replays(&modified_server_tick);
    //             for (_command_tick, command) in replay_commands {
    //                 shared_behavior::process_command(&command, &mut client_position);
    //             }
    //         }
    //     }
    // }
}

pub fn remove_component_events(mut event_reader: EventReader<RemoveComponentEvents>) {
    for events in event_reader.iter() {
        for (_entity, _component) in events.read::<Position>() {
            info!("removed Position component from entity");
        }
        for (_entity, _component) in events.read::<Color>() {
            info!("removed Color component from entity");
        }
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

use std::cmp::Ordering;

use bevy_ecs::{
    event::EventReader,
    system::{Commands, Query, ResMut},
};
use bevy_log::info;

use naia_bevy_server::{
    events::{
        AuthEvents, ConnectEvent, DespawnEntityEvent, DisconnectEvent, ErrorEvent,
        InsertComponentEvents, MessageEvents, RemoveComponentEvents, SpawnEntityEvent, TickEvent,
        UpdateComponentEvents,
    },
    CommandsExt, Random, Server,
};

use naia_bevy_demo_shared::{
    behavior as shared_behavior,
    channels::{
        EntityAssignmentChannel, GameSystemChannel, PlayerActionChannel, PlayerCommandChannel,
    },
    components::{
        hand::Hand,
        player::{Host, Player},
        server_hand::ServerHand,
        table::Table,
        Color, ColorValue, Counter, Position, Shape, ShapeValue,
    },
    messages::{Auth, EntityAssignment, KeyCommand, PlayCard, StartGame, UpdateTurn},
};

use crate::resources::Global;

pub fn auth_events(mut server: Server, mut event_reader: EventReader<AuthEvents>) {
    for events in event_reader.iter() {
        for (user_key, auth) in events.read::<Auth>() {
            if auth.username == "charlie" && auth.password == "12345" {
                // Accept incoming connection
                server.accept_connection(&user_key);
            } else {
                // Reject incoming connection
                server.reject_connection(&user_key);
            }
        }
    }
}

pub fn connect_events(
    mut commands: Commands,
    mut server: Server,
    mut global: ResMut<Global>,
    mut event_reader: EventReader<ConnectEvent>,
) {
    for ConnectEvent(user_key) in event_reader.iter() {
        let address = server
            .user_mut(user_key)
            // Add User to the main Room
            .enter_room(&global.main_room_key)
            // Get User's address for logging
            .address();

        info!("Naia Server connected to: {}", address);

        // Check if player is Host
        let player_num = global.users_map.len();

        info!("player_num {}", player_num);

        let player = Player::new(player_num);

        let entity = commands
            .spawn_empty()
            .enable_replication(&mut server)
            .insert(player)
            .id();

        if player_num == 0 {
            commands.entity(entity).insert(Host);
        }

        global.users_map.insert(*user_key, entity);
        global.players_map.insert(player_num, entity);

        server.room_mut(&global.main_room_key).add_entity(&entity);

        // Send an Entity Assignment message to the User that owns the Square
        let mut assignment_message = EntityAssignment::new(true);
        assignment_message.entity.set(&server, &entity);

        server.send_message::<EntityAssignmentChannel, EntityAssignment>(
            user_key,
            &assignment_message,
        );

        // Create components for Entity to represent new player

        // Position component
        let position = {
            let x = 16 * ((Random::gen_range_u32(0, 40) as i16) - 20);
            let y = 16 * ((Random::gen_range_u32(0, 30) as i16) - 15);
            Position::new(x, y)
        };

        // Color component
        let color = {
            let color_value = match server.users_count() % 4 {
                0 => ColorValue::Yellow,
                1 => ColorValue::Red,
                2 => ColorValue::Blue,
                _ => ColorValue::Green,
            };
            Color::new(color_value)
        };

        // Shape component
        let shape = Shape::new(ShapeValue::Square);

        // Spawn entity
        let entity = commands
            // Spawn new Entity
            .spawn_empty()
            // MUST call this to begin replication
            .enable_replication(&mut server)
            // Insert Position component
            .insert(position)
            // Insert Color component
            .insert(color)
            // Insert Shape component
            .insert(shape)
            // return Entity id
            .id();

        server.room_mut(&global.main_room_key).add_entity(&entity);

        global.user_to_square_map.insert(*user_key, entity);
        global.square_to_user_map.insert(entity, *user_key);
        global.total_player += 1;

        // Send an Entity Assignment message to the User that owns the Square
        let mut assignment_message = EntityAssignment::new(true);
        assignment_message.entity.set(&server, &entity);

        server.send_message::<EntityAssignmentChannel, EntityAssignment>(
            user_key,
            &assignment_message,
        );
    }
}

pub fn disconnect_events(
    mut commands: Commands,
    mut server: Server,
    mut global: ResMut<Global>,
    mut event_reader: EventReader<DisconnectEvent>,
) {
    for DisconnectEvent(user_key, user) in event_reader.iter() {
        info!("Naia Server disconnected from: {:?}", user.address);

        if let Some(entity) = global.users_map.remove(user_key) {
            commands.entity(entity).despawn();
            server
                .room_mut(&global.main_room_key)
                .remove_entity(&entity);
            info!("total player: {}", global.users_map.len());
        }

        if let Some(entity) = global.user_to_square_map.remove(user_key) {
            global.square_to_user_map.remove(&entity);
            commands.entity(entity).despawn();
            server
                .room_mut(&global.main_room_key)
                .remove_entity(&entity);
        }
        if let Some(client_entity) = global.user_to_cursor_map.remove(user_key) {
            if let Some(server_entity) = global.client_to_server_cursor_map.remove(&client_entity) {
                commands.entity(server_entity).despawn();
                server
                    .room_mut(&global.main_room_key)
                    .remove_entity(&server_entity);
            }
        }
    }
}

pub fn error_events(mut event_reader: EventReader<ErrorEvent>) {
    for ErrorEvent(error) in event_reader.iter() {
        info!("Naia Server Error: {:?}", error);
    }
}

pub fn message_events(
    mut commands: Commands,
    mut server: Server,
    mut event_reader: EventReader<MessageEvents>,
    mut global: ResMut<Global>,
    mut table_q: Query<&mut Table>,
    mut player_q: Query<&mut Player>,
    mut counter_q: Query<&mut Counter>,
    mut serverhand_q: Query<&mut ServerHand>,
) {
    for events in event_reader.iter() {
        for (_, _) in events.read::<PlayerActionChannel, StartGame>() {
            let users_map = global.users_map.clone();

            // Add the table component to the room
            let server_table = Table::new("".to_string());
            let server_table_entity = commands
                .spawn_empty()
                .enable_replication(&mut server)
                .insert(server_table)
                .id();

            let is_decided_first_play = false;

            let server_counter = Counter::default();

            let server_counter_entity = commands
                .spawn_empty()
                .enable_replication(&mut server)
                .insert(server_counter)
                .id();

            server
                .room_mut(&global.main_room_key)
                .add_entity(&server_table_entity)
                .add_entity(&server_counter_entity);

            // Draw card to players and start the game
            for (user_key, entity) in users_map.iter() {
                let hand = Hand {
                    cards: global.deck.deal(13),
                };

                // Calculate what player can play at first
                // if hand.contain_3_c() {
                //     info!("Player {:?}, take the first play", *p.pos);
                //     is_decided_first_play = true;
                //     commands.entity(*entity).insert(Active);
                // }

                let cards_str = hand.to_string();
                let server_hand = ServerHand::new(cards_str);

                commands.entity(*entity).insert(server_hand);

                server
                    .send_message::<GameSystemChannel, StartGame>(user_key, &StartGame::default());
            }

            if !is_decided_first_play {
                // TODO: decide who take the first turn!
            }
        }

        events
            .read::<PlayerActionChannel, PlayCard>()
            .into_iter()
            .for_each(|(user_key, cards_str)| {
                let put_hand = Hand::from_str(&cards_str.0);
                info!("HAND: {}", put_hand);
                if !put_hand.check_combination() {
                    // TODO: Should send an error messsage
                    return;
                }

                // Check if is their turn?
                let cur_player_entity = global.users_map.get(&user_key).unwrap();

                let cur_player = player_q.get(*cur_player_entity).unwrap();

                if !*cur_player.active {
                    // TODO: This player is not active!!!
                    info!("This player is not active!!!");
                    return;
                }

                // Update cards on the table
                let mut table = table_q.get_single_mut().unwrap();
                *table.cards = put_hand.to_string();

                // Keep track the history of the card being played
                if let Some(last_played_hand) = global.table.back() {
                    info!("In the check lasted {} \n {}", last_played_hand, put_hand);
                    if last_played_hand.cmp(&put_hand) == Ordering::Greater {
                        info!("Less!!!")
                    }
                }

                // Update cards of the player
                if let Ok(mut server_hand) = serverhand_q.get_mut(*cur_player_entity) {
                    info!("Update the card now:");
                    let hand_str = server_hand.cards.clone();
                    let mut player_hand = Hand::from(hand_str);
                    info!("server hand after 1: {}", player_hand.to_string());
                    info!("put hands: {}", put_hand.to_string());
                    // info!("put hands cards: {}", put_hand.car());

                    // remove cards
                    player_hand.remove_cards(put_hand.cards.as_slice());

                    info!("server hand after 2: {}", player_hand.to_string());
                    *server_hand.cards = player_hand.to_string();

                    info!("server hand after: {}", player_hand.to_string());

                    for (u_key, _) in global.users_map.iter() {
                        server.send_message::<GameSystemChannel, UpdateTurn>(
                            u_key,
                            &UpdateTurn::default(),
                        );
                    }

                    info!("server hand after Sended!");
                }

                global.table.push_back(put_hand);

                // TODO: update turn
                let total_player = global.total_player;
                let cur_active_pos = global.cur_active_pos;

                let next_active_pos = (cur_active_pos + 1) % total_player;

                info!(
                    "total: {:?}, cur: {:?}, next: {:?}",
                    total_player, cur_active_pos, next_active_pos
                );

                for mut player in player_q.iter_mut() {
                    *player.active = false;
                    if next_active_pos == *player.pos {
                        *player.active = true;
                        global.cur_active_pos = next_active_pos;
                    }
                }

                // reset counter
                if let Ok(mut counter) = counter_q.get_single_mut() {
                    counter.recount();
                }
            });
    }
}

pub fn tick_events(
    mut server: Server,
    mut position_query: Query<&mut Position>,
    mut tick_reader: EventReader<TickEvent>,
) {
    let mut has_ticked = false;

    for TickEvent(server_tick) in tick_reader.iter() {
        has_ticked = true;

        // All game logic should happen here, on a tick event
        let mut messages = server.receive_tick_buffer_messages(server_tick);

        for (_user_key, key_command) in messages.read::<PlayerCommandChannel, KeyCommand>() {
            let Some(entity) = &key_command.entity.get(&server) else {
                continue;
            };
            let Ok(mut position) = position_query.get_mut(*entity) else {
                continue;
            };
            shared_behavior::process_command(&key_command, &mut position);
        }
    }

    if has_ticked {
        // Update scopes of entities
        for (_, user_key, entity) in server.scope_checks() {
            // You'd normally do whatever checks you need to in here..
            // to determine whether each Entity should be in scope or not.

            // This indicates the Entity should be in this scope.
            server.user_scope(&user_key).include(&entity);

            // And call this if Entity should NOT be in this scope.
            // server.user_scope(..).exclude(..);
        }
    }
}

pub fn spawn_entity_events(mut event_reader: EventReader<SpawnEntityEvent>) {
    for SpawnEntityEvent(_, _) in event_reader.iter() {
        info!("spawned client entity");
    }
}

pub fn despawn_entity_events(mut event_reader: EventReader<DespawnEntityEvent>) {
    for DespawnEntityEvent(_, _) in event_reader.iter() {
        info!("despawned client entity");
    }
}

pub fn insert_component_events(
    mut commands: Commands,
    mut server: Server,
    mut global: ResMut<Global>,
    mut event_reader: EventReader<InsertComponentEvents>,
    position_query: Query<&Position>,
) {
    for events in event_reader.iter() {
        for (user_key, client_entity) in events.read::<Position>() {
            info!("insert component into client entity");

            if let Ok(client_position) = position_query.get(client_entity) {
                // New Position Component
                let server_position = Position::new(*client_position.x, *client_position.y);

                // New Color component
                let color = {
                    let color_value = match server.users_count() % 4 {
                        0 => ColorValue::Yellow,
                        1 => ColorValue::Red,
                        2 => ColorValue::Blue,
                        _ => ColorValue::Green,
                    };
                    Color::new(color_value)
                };

                // New Shape component
                let shape = Shape::new(ShapeValue::Circle);

                // Spawn entity
                let server_entity = commands
                    // Spawn new Square Entity
                    .spawn_empty()
                    // MUST call this to begin replication
                    .enable_replication(&mut server)
                    // Insert Position component
                    .insert(server_position)
                    // Insert Color component
                    .insert(color)
                    // Insert Shape component
                    .insert(shape)
                    // return Entity id
                    .id();

                server
                    .room_mut(&global.main_room_key)
                    .add_entity(&server_entity);

                global.user_to_cursor_map.insert(user_key, client_entity);
                global
                    .client_to_server_cursor_map
                    .insert(client_entity, server_entity);
            }
        }
    }
}

pub fn update_component_events(
    global: ResMut<Global>,
    mut event_reader: EventReader<UpdateComponentEvents>,
    mut position_query: Query<&mut Position>,
) {
    for events in event_reader.iter() {
        for (_user_key, client_entity) in events.read::<Position>() {
            if let Some(server_entity) = global.client_to_server_cursor_map.get(&client_entity) {
                if let Ok([client_position, mut server_position]) =
                    position_query.get_many_mut([client_entity, *server_entity])
                {
                    server_position.x.mirror(&client_position.x);
                    server_position.y.mirror(&client_position.y);
                }
            }
        }
    }
}

pub fn remove_component_events(mut event_reader: EventReader<RemoveComponentEvents>) {
    for events in event_reader.iter() {
        for (_user_key, _entity, _component) in events.read::<Position>() {
            info!("removed Position component from client entity");
        }
    }
}

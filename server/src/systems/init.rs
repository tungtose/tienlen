use std::collections::{HashMap, VecDeque};

use bevy_ecs::system::Commands;
use bevy_log::info;

use naia_bevy_server::{transport::webrtc, Server};

use crate::resources::{Global, PlayerMap};

use naia_bevy_demo_shared::messages::Counter;

pub fn init(mut commands: Commands, mut server: Server) {
    info!("Tienlen server is running");

    let server_addresses = webrtc::ServerAddrs::new(
        "127.0.0.1:14191"
            .parse()
            .expect("could not parse Signaling address/port"),
        // IP Address to listen on for UDP WebRTC data channels
        "127.0.0.1:14192"
            .parse()
            .expect("could not parse WebRTC data address/port"),
        // The public WebRTC IP address to advertise
        "http://127.0.0.1:14192",
    );
    let socket = webrtc::Socket::new(&server_addresses, server.socket_config());
    server.listen(socket);

    // Create a new, singular room, which will contain Users and Entities that they
    // can receive updates from
    let main_room_key = server.make_room().key();

    let table = VecDeque::new();

    let counter = Counter::new(0.);
    let players_map = PlayerMap::new();

    let player_data_map = HashMap::new();

    // Init Global Resource
    let global = Global {
        counter,
        player_data_map,
        time: 0.,
        leader_turn: true,
        table,
        players_map,
        main_room_key,
        total_player: 0,
        cur_active_pos: 0,
        users_map: HashMap::new(),
        user_to_square_map: HashMap::new(),
        user_to_cursor_map: HashMap::new(),
        client_to_server_cursor_map: HashMap::new(),
        square_to_user_map: HashMap::new(),
    };

    // Insert Global Resource
    commands.insert_resource(global);
}

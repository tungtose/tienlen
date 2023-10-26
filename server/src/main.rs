use bevy_app::{App, ScheduleRunnerPlugin, Startup, Update};
use bevy_core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_log::{info, LogPlugin};
use bevy_time::TimePlugin;
use naia_shared::ConnectionConfig;
use std::time::Duration;

use naia_bevy_demo_shared::protocol;
use naia_bevy_server::{Plugin as ServerPlugin, ReceiveEvents, ServerConfig};

mod resources;
mod systems;

use systems::{events, init};

use crate::systems::common;

fn main() {
    info!("Naia Bevy Server Demo starting up");

    let connection_config = ConnectionConfig {
        disconnection_timeout_duration: Duration::from_secs(5),
        ..Default::default()
    };

    let server_config = ServerConfig {
        connection: connection_config,
        ..Default::default()
    };

    // Build App
    App::default()
        // Plugins
        .add_plugins((
            TaskPoolPlugin::default(),
            TimePlugin::default(),
            TypeRegistrationPlugin::default(),
            FrameCountPlugin::default(),
            LogPlugin::default(),
            ServerPlugin::new(server_config, protocol()),
            ScheduleRunnerPlugin::run_loop(Duration::from_millis(3)),
        ))
        // Startup System
        .add_systems(Startup, (init, common::set_up_counter))
        // Test
        .add_systems(
            Update,
            (
                common::countdown,
                common::run_out_countdown,
                events::end_match,
            ),
        )
        // Receive Server Events
        .add_systems(
            Update,
            (
                events::auth_events,
                events::connect_events,
                events::disconnect_events,
                events::error_events,
                events::accept_start_game,
                events::message_events,
                events::tick_events,
                events::spawn_entity_events,
                events::despawn_entity_events,
                events::insert_component_events,
                events::update_component_events,
                events::remove_component_events,
            )
                .chain()
                .in_set(ReceiveEvents),
        )
        // Run App
        .run();
}

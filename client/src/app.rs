use std::time::Duration;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    window::{Window, WindowPlugin},
};
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, ReceiveEvents};
use naia_bevy_demo_shared::protocol;
use naia_shared::ConnectionConfig;

use crate::{
    fps::ScreenDiagsTextPlugin,
    // assets::AssetPlugin,
    game::GamePlugin,
    states::MainState,
    system_set::{MainLoop, SystemSetsPlugin, Tick},
    systems::{events, init, my_cursor_system, sync},
    ui::UiPlugin,
    welcome::WelcomeScreenPlugin,
};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub fn run() {
    let window_plug = WindowPlugin {
        primary_window: Some(Window {
            title: "thirteen".into(),
            canvas: Some("#thirteen".into()),
            resolution: (960., 600.).into(),
            fit_canvas_to_parent: false,
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    };

    let log_plug = LogPlugin {
        filter: "bevy_mod_picking=error,wgpu_core=error".into(),
        level: Level::INFO,
    };

    let asset_plug = AssetPlugin {
        watch_for_changes_override: Some(true),
        ..default()
    };

    let client_config = ClientConfig {
        connection: ConnectionConfig {
            disconnection_timeout_duration: Duration::from_secs(5),
            ..Default::default()
        },
        ..Default::default()
    };

    App::default()
        // Bevy Plugins
        .add_state::<MainState>()
        .add_plugins(
            DefaultPlugins
                .set(window_plug)
                .set(log_plug)
                .set(asset_plug),
        )
        // Add Naia Client Plugin
        .add_plugins(ClientPlugin::new(client_config, protocol()))
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(UiPlugin)
        .add_plugins(ScreenDiagsTextPlugin)
        .add_plugins(WelcomeScreenPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(SystemSetsPlugin)
        .add_plugins(crate::assets::AssetPlugin)
        // Background Color
        .insert_resource(ClearColor(Color::BLACK))
        // Startup System
        .add_systems(Startup, init)
        .add_systems(Update, my_cursor_system)
        // Receive Client Events
        .add_systems(
            Update,
            (
                events::connect_events,
                events::disconnect_events,
                events::reject_events,
                events::spawn_entity_events,
                events::despawn_entity_events,
                events::message_events,
            )
                .chain()
                .in_set(ReceiveEvents),
        )
        // Tick Event
        .add_systems(Update, events::tick_events.in_set(Tick))
        .add_systems(
            Update,
            (
                // input::key_input,
                // input::cursor_input,
                sync::sync_main_player,
                sync::sync_foreign_player,
                // sync::sync_main_player_cards,
                // sync::sync_table_cards,
                sync::sync_timer,
                sync::sync_player,
            )
                .chain()
                .in_set(MainLoop),
        )
        // Run App
        .run();
}

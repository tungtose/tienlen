use bevy::{
    prelude::*,
    window::{Window, WindowPlugin},
};
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, ReceiveEvents};
use naia_bevy_demo_shared::protocol;

use crate::{
    assets::AssetPlugin,
    game::GamePlugin,
    states::MainState,
    systems::{events, init, input, my_cursor_system, sync},
    ui::UiPlugin,
    welcome::WelcomeScreenPlugin,
};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct MainLoop;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct Tick;

pub fn run() {
    App::default()
        // Bevy Plugins
        .add_state::<MainState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "thirteen".into(),
                canvas: Some("#thirteen".into()),
                resolution: (800., 500.).into(),
                fit_canvas_to_parent: false,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        // Add Naia Client Plugin
        .add_plugins(ClientPlugin::new(ClientConfig::default(), protocol()))
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(UiPlugin)
        .add_plugins(WelcomeScreenPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(AssetPlugin)
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
        .configure_set(Update, Tick.after(ReceiveEvents))
        .add_systems(Update, events::tick_events.in_set(Tick))
        // Realtime Gameplay Loop
        .configure_set(Update, MainLoop.after(Tick))
        .add_systems(
            Update,
            (
                input::key_input,
                input::cursor_input,
                sync::sync_main_player,
                sync::sync_foreign_player,
                sync::sync_main_player_cards,
                sync::sync_table_cards,
                sync::sync_timer,
                sync::sync_player,
            )
                .chain()
                .in_set(MainLoop),
        )
        // Run App
        .run();
}

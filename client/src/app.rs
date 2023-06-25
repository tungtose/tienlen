use bevy::{
    prelude::{
        App, ClearColor, Color, IntoSystemConfig, IntoSystemConfigs, IntoSystemSetConfig, SystemSet,
    },
    DefaultPlugins,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, ReceiveEvents};
use naia_bevy_demo_shared::protocol;

use crate::{
    assets::AssetPlugin,
    game::GamePlugin,
    states::MainState,
    systems::{events, init, input, sync},
    ui::UiPlugin,
};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct MainLoop;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct Tick;

pub fn run() {
    App::default()
        // Bevy Plugins
        .add_state::<MainState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        // Add Naia Client Plugin
        .add_plugin(ClientPlugin::new(ClientConfig::default(), protocol()))
        .add_plugin(UiPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(AssetPlugin)
        // Background Color
        .insert_resource(ClearColor(Color::BLACK))
        // Startup System
        .add_startup_system(init)
        // Receive Client Events
        .add_systems(
            (
                events::connect_events,
                events::disconnect_events,
                events::reject_events,
                events::spawn_entity_events,
                events::despawn_entity_events,
                events::insert_component_events,
                events::update_component_events,
                events::remove_component_events,
                events::message_events,
                // common::test,
            )
                .chain()
                .in_set(ReceiveEvents),
        )
        // Tick Event
        .configure_set(Tick.after(ReceiveEvents))
        .add_system(events::tick_events.in_set(Tick))
        // Realtime Gameplay Loop
        .configure_set(MainLoop.after(Tick))
        .add_systems(
            (
                input::key_input,
                input::cursor_input,
                sync::sync_clientside_sprites,
                sync::sync_serverside_sprites,
                sync::sync_cursor_sprite,
            )
                .chain()
                .in_set(MainLoop),
        )
        // Run App
        .run();
}

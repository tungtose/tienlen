use bevy::{prelude::*, render::RenderSet};
use naia_bevy_client::ReceiveEvents;

/// Configures the system sets of the game, defining their order of execution.
pub struct SystemSetsPlugin;

impl Plugin for SystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(Update, Playing.before(Animating))
            .configure_set(Update, Playing.before(Waiting))
            .configure_set(Update, Animating.before(WaitingAnotherPlayer))
            .configure_set(Update, Animating.after(ReceiveEvents))
            .configure_set(Update, Animating.after(RenderSet::Render))
            .configure_set(Update, Tick.after(ReceiveEvents))
            // Realtime Gameplay Loop
            .configure_set(Update, MainLoop.after(Tick));
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MainLoop;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Tick;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Playing;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Waiting;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaitingAnotherPlayer;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Animating;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UpdateGameState;

use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum MainState {
    #[default]
    LoadAssets,
    Welcome,
    Lobby,
    Wait,
    Game,
}

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum GameState {
    #[default]
    None,
    PlayerInput,
    TurnUpdate,
}

// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub enum TurnSet {
//     Logic,
//     Animation,
//     Tick,
// }

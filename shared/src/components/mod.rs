use naia_bevy_shared::{Protocol, ProtocolPlugin};

pub mod card;
pub mod cards;
pub mod deck;
pub mod hand;
pub mod rank;
pub mod suit;

pub mod player;
pub mod table;
pub mod timer;
pub mod turn;
pub use player::Player;

mod color;
pub use color::{Color, ColorValue};

mod position;
pub use position::Position;

mod shape;
pub use shape::{Shape, ShapeValue};

pub use {
    player::{Active, Host},
    table::Table,
    timer::{Counter, PrestartCounter, TurnCounter},
};

// Plugin
pub struct ComponentsPlugin;

impl ProtocolPlugin for ComponentsPlugin {
    fn build(&self, protocol: &mut Protocol) {
        protocol
            .add_component::<Color>()
            .add_component::<Position>()
            .add_component::<Shape>()
            .add_component::<Player>()
            .add_component::<Active>()
            .add_component::<Table>()
            .add_component::<Counter>()
            .add_component::<Host>();
    }
}

use naia_bevy_shared::{Protocol, ProtocolPlugin};

mod auth;
mod counter;
mod entity_assignment;
mod game;
mod key_command;

pub use auth::Auth;
pub use counter::Counter;
pub use entity_assignment::EntityAssignment;
pub use game::StartGame;
pub use key_command::KeyCommand;

// Plugin
pub struct MessagesPlugin;

impl ProtocolPlugin for MessagesPlugin {
    fn build(&self, protocol: &mut Protocol) {
        protocol
            .add_message::<Auth>()
            .add_message::<EntityAssignment>()
            .add_message::<KeyCommand>()
            .add_message::<Counter>()
            .add_message::<StartGame>();
    }
}

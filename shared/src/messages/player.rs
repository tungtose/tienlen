use naia_bevy_shared::Message;

#[derive(Message, Default, Debug)]
pub struct PlayerMessage(pub usize, pub String);

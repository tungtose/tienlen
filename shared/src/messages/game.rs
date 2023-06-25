use naia_bevy_shared::Message;

#[derive(Message, Debug, Default)]
pub struct StartGame;

#[derive(Message, Debug, Default)]
pub struct PlayCard(pub String);

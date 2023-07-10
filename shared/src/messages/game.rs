use naia_bevy_shared::Message;

#[derive(Message, Debug, Default)]
pub struct StartGame;

#[derive(Message, Debug, Default)]
pub struct UpdatePlayerCards;

#[derive(Message, Debug, Default)]
pub struct PlayCard(pub String);

#[derive(Message, Debug, Default)]
pub struct UpdateTurn;

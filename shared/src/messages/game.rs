use naia_bevy_shared::Message;

#[derive(Message, Default, Debug)]
pub struct PlayerMessage {
    pub pos: usize,
    pub active: bool,
    pub cards: String,
    pub score: u32,
}

#[derive(Message, Default)]
pub struct StartGame(pub String);

#[derive(Message, Debug, Default)]
pub struct UpdatePlayerCards;

#[derive(Message, Debug, Default)]
pub struct PlayCard(pub String);

#[derive(Message, Debug, Default)]
pub struct SkipTurn;

#[derive(Message, Debug, Default)]
pub struct UpdateTurn(pub usize);

#[derive(Message, Debug, Default)]
pub struct UpdateScore(pub u32);

#[derive(Message, Debug, Default)]
pub struct NewMatch;

#[derive(Message, Debug, Default)]
pub struct NewPlayer(pub String);

#[derive(Message, Debug, Default)]
pub struct PlayerReady;

#[derive(Message, Debug, Default)]
pub struct NewPlayerJoin;

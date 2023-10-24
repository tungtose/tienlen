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

#[derive(Message, Default)]
pub struct RequestStart;

#[derive(Message, Default)]
pub struct WaitForStart(pub usize);

#[derive(Message, Default)]
pub struct AcceptStartGame {
    pub cards: String,
    pub active_player: usize,
}

#[derive(Message, Debug, Default)]
pub struct UpdatePlayerCards;

#[derive(Message, Debug, Default)]
pub struct PlayCard(pub String);

#[derive(Message, Debug, Default)]
pub struct AcceptPlayCard {
    pub cur_player: usize,
    pub cards: String,
    pub next_player: usize,
    pub run_out_card: bool,
}

#[derive(Message, Debug, Default)]
pub struct SkipTurn;

#[derive(Message, Debug, Default)]
pub struct UpdateTurn(pub usize);

#[derive(Message, Debug, Default)]
pub struct UpdateScore(pub u32);

#[derive(Message, Debug, Default)]
pub struct NewMatch {
    pub cards: String,
    pub active_player: usize,
}

#[derive(Message, Debug, Default)]
pub struct NewPlayer(pub String);

#[derive(Message, Debug, Default)]
pub struct PlayerReady;

#[derive(Message, Debug, Default)]
pub struct AcceptPlayerReady {
    pub name: String,
    pub server_pos: usize,
}

#[derive(Message, Debug, Default)]
pub struct NewPlayerJoin;

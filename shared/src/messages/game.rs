use naia_bevy_shared::Message;

#[derive(Message, Debug)]
pub struct Game {
    start_game: bool,
}

impl Game {
    pub fn new(start_game: bool) -> Self {
        Self { start_game }
    }
}

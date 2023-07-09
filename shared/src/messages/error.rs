use naia_bevy_shared::Message;

#[derive(Message, Debug)]
pub struct ErrorCode {
    code: usize,
}

impl Default for ErrorCode {
    fn default() -> Self {
        Self { code: 0 }
    }
}

pub enum GameError {
    InvalidCards(&'static str),
    WrongTurn(&'static str),
    WrongCombination(&'static str),
    UnknownError,
}

impl From<GameError> for ErrorCode {
    fn from(game_error: GameError) -> Self {
        match game_error {
            GameError::InvalidCards(_) => Self { code: 0 },
            GameError::WrongCombination(_) => Self { code: 1 },
            GameError::WrongTurn(_) => Self { code: 2 },
            GameError::UnknownError => todo!(),
        }
    }
}

impl From<ErrorCode> for GameError {
    fn from(error_code: ErrorCode) -> Self {
        match error_code.code {
            0 => Self::InvalidCards("Your cards is not big enough!"),
            1 => Self::WrongCombination("Wrong Combination!"),
            2 => Self::WrongTurn("Not in your turn yet!"),
            _ => Self::UnknownError,
        }
    }
}

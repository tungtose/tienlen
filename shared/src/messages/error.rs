use naia_bevy_shared::Message;

#[derive(Message, Debug, Default)]
pub struct ErrorCode {
    code: usize,
}

// impl Default for ErrorCode {
//     fn default() -> Self {
//         Self { code: 0 }
//     }
// }

pub enum GameError {
    InvalidCards,
    WrongTurn,
    WrongCombination,
    UnknownError,
}

impl From<GameError> for ErrorCode {
    fn from(game_error: GameError) -> Self {
        match game_error {
            GameError::InvalidCards => Self { code: 0 },
            GameError::WrongCombination => Self { code: 1 },
            GameError::WrongTurn => Self { code: 2 },
            GameError::UnknownError => todo!(),
        }
    }
}

impl From<ErrorCode> for GameError {
    fn from(error_code: ErrorCode) -> Self {
        match error_code.code {
            0 => Self::InvalidCards,
            1 => Self::WrongCombination,
            2 => Self::WrongTurn,
            _ => Self::UnknownError,
        }
    }
}

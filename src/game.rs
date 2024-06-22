use crate::{board::Board, tile::Tile};

pub struct Game {
    pub board: Board,
    pub current_player: Tile,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_player: Tile::Player1,
        }
    }
}

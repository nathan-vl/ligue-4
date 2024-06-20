use crate::{board::Board, tile::Tile};

struct Game {
    board: Board,
    current_player: Tile,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_player: Tile::Player1,
        }
    }
}

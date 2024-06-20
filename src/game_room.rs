use std::net::TcpStream;

use crate::board::Board;

pub struct GameRoom {
    board: Board,
    player1: TcpStream,
    player2: Option<TcpStream>,
}

impl GameRoom {
    pub fn new(player1: TcpStream) -> Self {
        Self {
            board: Board::new(),
            player1,
            player2: None,
        }
    }
}

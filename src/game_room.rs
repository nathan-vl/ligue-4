use std::net::TcpStream;

use crate::{board::Board, tile::Tile};

#[derive(Debug)]
pub struct GameRoom {
    pub board: Board,
    current_player: Tile,
    pub player1: TcpStream,
    pub player2: Option<TcpStream>,
}

impl GameRoom {
    pub fn new(player1: TcpStream) -> Self {
        Self {
            board: Board::new(),
            current_player: Tile::Player1,
            player1,
            player2: None,
        }
    }

    pub fn available_rooms(rooms: &mut [GameRoom]) -> Vec<&mut GameRoom> {
        rooms
            .iter_mut()
            .filter(|room| room.player2.is_none())
            .collect()
    }
}

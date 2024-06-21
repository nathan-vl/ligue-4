use std::net::TcpStream;

use crate::game::Game;

pub struct GameRoom {
    pub player1: TcpStream,
    pub player2: Option<TcpStream>,

    pub game: Option<Game>,
}

impl GameRoom {
    pub fn new(player1: TcpStream) -> Self {
        Self {
            player1,
            player2: None,
            game: None,
        }
    }

    pub fn available_rooms(rooms: &mut [GameRoom]) -> Vec<&mut GameRoom> {
        rooms
            .iter_mut()
            .filter(|room| room.player2.is_none())
            .collect()
    }
}

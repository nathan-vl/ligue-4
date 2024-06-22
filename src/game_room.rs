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

    pub fn first_available_room(rooms: &mut Vec<GameRoom>) -> Option<GameRoom> {
        for (i, room) in rooms.iter_mut().enumerate() {
            if room.player2.is_none() {
                let r = rooms.remove(i);
                return Some(r);
            }
        }
        None
    }
}

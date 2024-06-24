use crate::{game::Game, player::Player};

pub struct GameRoom {
    pub player1: Player,
    pub player2: Option<Player>,

    pub game: Option<Game>,
}

impl GameRoom {
    pub fn new(player1: Player) -> Self {
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

use serde::{Deserialize, Serialize};

use crate::{board::Board, tile::Tile};

#[derive(Deserialize, Debug, Serialize)]

pub enum Response {
    PlayerCreatedRoom,

    PlayerEnteredRoom { board: Board },
    AnotherPlayerJoinedRoom { board: Board },

    CurrentGameState { board: Board, current_player: Tile },
}

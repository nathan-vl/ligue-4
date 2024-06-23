use serde::{Deserialize, Serialize};

use crate::{board::Board, tile::Tile};

#[derive(Deserialize, Debug, Serialize)]

pub enum Response {
    CreatedRoom,

    JoinedRoom { player_tile: Tile },
    AnotherPlayerJoinedRoom { player_tile: Tile },

    AskTurn { board: Board },
    AnotherPlayerTurn { board: Board },

    PlayerWin { board: Board },
    PlayerLost { board: Board },
    Draw { board: Board },

    OtherPlayerDisconnected,
    InvalidRequest { message: String },
}

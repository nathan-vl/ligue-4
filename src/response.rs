use serde::{Deserialize, Serialize};

use crate::{board::Board, tile::Tile};

#[derive(Deserialize, Debug, Serialize)]

pub enum Response {
    CreatedRoom,

    JoinedRoom {
        player_tile: Tile,
        other_player_name: String,
    },
    AnotherPlayerJoinedRoom {
        player_tile: Tile,
        another_player_name: String,
    },

    AskTurn {
        board: Board,
    },
    AnotherPlayerTurn {
        board: Board,
    },

    PlayerWin {
        board: Board,
    },
    PlayerLost {
        board: Board,
    },
    Draw {
        board: Board,
    },

    OtherPlayerDisconnected,
    InvalidRequest {
        message: String,
    },
}

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Tile {
    Player1,
    Player2,
}

impl Tile {
    pub fn opposite(&self) -> Self {
        if *self == Tile::Player1 {
            Tile::Player2
        } else {
            Tile::Player1
        }
    }

    pub fn to_number(self) -> u8 {
        match self {
            Tile::Player1 => 1,
            Tile::Player2 => 2,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tile = match self {
            Tile::Player1 => "\x1b[0;31mo\x1b[0m",
            Tile::Player2 => "\x1b[0;34mo\x1b[0m",
        };
        write!(f, "{}", tile)
    }
}

#[derive(Clone, Copy, PartialEq)]
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

    pub fn display_tile(tile: &Option<Self>) -> &str {
        match tile {
            Some(player) => match player {
                Tile::Player1 => "1",
                Tile::Player2 => "2",
            },
            None => " ",
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    NewPlayer { name: String },
    Play { column: u8 },
    Rematch { accept: String },
}

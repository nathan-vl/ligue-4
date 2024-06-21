use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    NewPlayer,
    Play { column: u8 },
}

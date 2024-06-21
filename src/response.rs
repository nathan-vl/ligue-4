use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub enum Response {
    JoinedRoom { message: String },
    CurrentGameState,
}

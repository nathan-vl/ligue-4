use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    New,
    Play(i64, u8), // id da sala, coluna da ação
}

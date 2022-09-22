use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pedido {
    pub agua: u32,
    pub cafe: u32,
    pub espuma: u32,
}

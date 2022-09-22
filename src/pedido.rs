use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pedido {
    /// Cantidad de agua del 1 al 10
    pub agua: u32,
    /// Cantidad de cafe del 1 al 10
    pub cafe: u32,
    /// Cantidad de espuma del 1 al 10
    pub espuma: u32,
}

use rand::Rng;
use std::fs::File;
use std::io::Write;

use crate::constantes::{MIN_CANTIDAD, MAX_CANTIDAD};
use crate::error::CafeteriaError;

const CANT_PEDIDOS: usize = 100;

#[derive(Debug)]
pub struct Pedido {
    pub id: usize,
    /// Cantidad de agua
    pub agua: u32,
    /// Cantidad de cafe
    pub cafe: u32,
    /// Cantidad de espuma
    pub espuma: u32,
}

impl Pedido {
    pub fn new(id: usize, agua: u32, cafe: u32, espuma: u32) -> Result<Pedido, CafeteriaError> {
        if !(MIN_CANTIDAD..=MAX_CANTIDAD).contains(&agua) ||
            !(MIN_CANTIDAD..=MAX_CANTIDAD).contains(&cafe) ||
            !(MIN_CANTIDAD..=MAX_CANTIDAD).contains(&espuma) {
            Err(CafeteriaError::PedidoInvalido)
        } else {
            Ok(Pedido { id, agua, cafe, espuma })
        }
    }

    pub fn new_random(id: usize) -> Pedido {
        Pedido {
            id,
            agua: rand::thread_rng().gen_range(MIN_CANTIDAD..=MAX_CANTIDAD),
            cafe: rand::thread_rng().gen_range(MIN_CANTIDAD..=MAX_CANTIDAD),
            espuma: rand::thread_rng().gen_range(MIN_CANTIDAD..=MAX_CANTIDAD),
        }
    }

    pub fn from_line(line: &str) -> Result<Pedido, CafeteriaError> {
        let mut pedido = line.split(',');
        let id = pedido.next().ok_or(CafeteriaError::PedidoInvalido)?.parse::<usize>()?;
        let agua = pedido.next().ok_or(CafeteriaError::PedidoInvalido)?.parse::<u32>()?;
        let cafe = pedido.next().ok_or(CafeteriaError::PedidoInvalido)?.parse::<u32>()?;
        let espuma = pedido.next().ok_or(CafeteriaError::PedidoInvalido)?.parse::<u32>()?;
        Pedido::new(id, agua, cafe, espuma)
    }

    pub fn to_line(&self) -> String {
        format!("{},{},{},{}", self.id, self.agua, self.cafe, self.espuma)
    }
}

pub fn generate_file(ruta: &str) -> Result<(), CafeteriaError> {
    let pedidos: Vec<Pedido> = (1..=CANT_PEDIDOS)
        .map(Pedido::new_random)
        .collect();

    let mut file = File::create(ruta).map_err(|_| CafeteriaError::CreacionArchivo)?;
    for pedido in pedidos {
        writeln!(file, "{}", pedido.to_line()).map_err(|_| CafeteriaError::EscrituraArchivo)?;
    }
    Ok(())
}

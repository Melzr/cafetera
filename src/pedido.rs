use rand::Rng;
use std::fs::File;
use std::io::Write;

use crate::constantes::{MAX_CANTIDAD, MIN_CANTIDAD};
use crate::error::CafeteriaError;

const CANT_PEDIDOS: usize = 100;

/// Información del pedido de un cliente.
///
/// Se representa como una línea en el archivo de pedidos de la siguiente manera:
///
/// ```
/// <id>,<agua>,<cafe>,<espuma>
/// ```
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
    /// # Errors
    /// * En caso de que agua, cafe o espuma no estén en el rango [[`MIN_CANTIDAD`], =[`MAX_CANTIDAD`]]
    /// devuelve [`CafeteriaError::PedidoInvalido`].
    pub fn new(id: usize, agua: u32, cafe: u32, espuma: u32) -> Result<Pedido, CafeteriaError> {
        if !(MIN_CANTIDAD..=MAX_CANTIDAD).contains(&agua)
            || !(MIN_CANTIDAD..=MAX_CANTIDAD).contains(&cafe)
            || !(MIN_CANTIDAD..=MAX_CANTIDAD).contains(&espuma)
        {
            Err(CafeteriaError::PedidoInvalido)
        } else {
            Ok(Pedido {
                id,
                agua,
                cafe,
                espuma,
            })
        }
    }

    /// Genera un pedido con cantidades aleatorias de agua, café y espuma.
    pub fn new_random(id: usize) -> Pedido {
        Pedido {
            id,
            agua: rand::thread_rng().gen_range(MIN_CANTIDAD..=MAX_CANTIDAD),
            cafe: rand::thread_rng().gen_range(MIN_CANTIDAD..=MAX_CANTIDAD),
            espuma: rand::thread_rng().gen_range(MIN_CANTIDAD..=MAX_CANTIDAD),
        }
    }

    /// Parsea una línea de un archivo de pedidos.
    ///
    /// # Errors
    /// * En caso de que agua, cafe o espuma no estén en el rango [[`MIN_CANTIDAD`], =[`MAX_CANTIDAD`]]
    /// devuelve [`CafeteriaError::PedidoInvalido`].
    /// * En caso de que la línea no tenga el formato correcto devuelve [`CafeteriaError::PedidoInvalido`].
    pub fn from_line(line: &str) -> Result<Pedido, CafeteriaError> {
        let mut pedido = line.split(',');
        let id = pedido
            .next()
            .ok_or(CafeteriaError::PedidoInvalido)?
            .parse::<usize>()?;
        let agua = pedido
            .next()
            .ok_or(CafeteriaError::PedidoInvalido)?
            .parse::<u32>()?;
        let cafe = pedido
            .next()
            .ok_or(CafeteriaError::PedidoInvalido)?
            .parse::<u32>()?;
        let espuma = pedido
            .next()
            .ok_or(CafeteriaError::PedidoInvalido)?
            .parse::<u32>()?;
        Pedido::new(id, agua, cafe, espuma)
    }

    /// Parseo a String.
    pub fn to_line(&self) -> String {
        format!("{},{},{},{}", self.id, self.agua, self.cafe, self.espuma)
    }
}

/// Genera un archivo de pedidos con [`CANT_PEDIDOS`] pedidos aleatorios en la ruta dada.
pub fn generate_file(ruta: &str) -> Result<(), CafeteriaError> {
    let pedidos: Vec<Pedido> = (1..=CANT_PEDIDOS).map(Pedido::new_random).collect();

    let mut file = File::create(ruta).map_err(|_| CafeteriaError::CreacionArchivo)?;
    for pedido in pedidos {
        writeln!(file, "{}", pedido.to_line()).map_err(|_| CafeteriaError::EscrituraArchivo)?;
    }
    Ok(())
}

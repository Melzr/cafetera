use std::cmp::min;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use crate::constantes::{E, L, MAX_CANTIDAD, TIEMPO_ESPUMA};
use crate::error::CafeteriaError;

pub struct ContenedorEspuma {
    /// Cantidad actual de espuma
    pub espuma: u32,
    /// Cantidad actual de leche
    pub leche: u32,
    /// true si no quedan pedidos por realizar
    pub fin: bool,
    /// Cantidad total de espuma consumida
    pub espuma_consumida: u32,
    /// Cantidad total de leche consumida
    pub leche_consumida: u32,
}

impl ContenedorEspuma {
    #[must_use]
    pub fn new() -> Self {
        ContenedorEspuma {
            espuma: 0,
            leche: L,
            fin: false,
            espuma_consumida: 0,
            leche_consumida: 0,
        }
    }
}

impl Default for ContenedorEspuma {
    fn default() -> Self {
        Self::new()
    }
}

/// Loop donde se rellena el contenedor de espuma cuando su cantidad sea menor a [`MAX_CANTIDAD`]
/// y el contenedor se encuentre disponible. Se rellena por completo en [`TIEMPO_ESPUMA`] milisegundos,
/// durante este tiempo no se podrá utilizar el dispensador de espuma.
/// También se rellena el contenedor de leche cuando su cantidad sea menor a [`E`], esto es instantáneo.
/// Finaliza cuando [`ContenedorEspuma`].fin es true.
///
/// # Errors
/// * En caso de que el lock del contenedor se encuentre envenenado, devuelve [`CafeteriaError::LockEnvenenado`].
pub fn rellenar_espuma(
    contenedor: Arc<(Mutex<ContenedorEspuma>, Condvar)>,
) -> Result<(), CafeteriaError> {
    let (espuma_lock, espuma_cvar) = &*contenedor;
    loop {
        if let Ok(mut state) = espuma_cvar.wait_while(espuma_lock.lock()?, |cont| {
            cont.espuma >= MAX_CANTIDAD && !cont.fin
        }) {
            if state.fin {
                break;
            }
            println!("[DEBUG] Reponiendo espuma");
            thread::sleep(Duration::from_millis(TIEMPO_ESPUMA));
            let cantidad = min(E - state.espuma, state.leche);
            state.espuma += cantidad;
            state.leche -= cantidad;
            state.leche_consumida += cantidad;
            if state.leche < E {
                println!(
                    "[INFO] Contenedor de leche por debajo del {}%. Reponiendo.",
                    E * 100 / L
                );
                state.leche = L;
            }
            espuma_cvar.notify_one();
        }
    }
    Ok(())
}

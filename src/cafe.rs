use std::cmp::min;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use crate::constantes::{C, G, MAX_CANTIDAD, TIEMPO_CAFE};
use crate::error::CafeteriaError;

pub struct ContenedorCafe {
    /// Cantidad actual de cafe molido
    pub cafe_molido: u32,
    /// Cantidad actual de granos de cafe
    pub granos: u32,
    /// true si no quedan pedidos por realizar
    pub fin: bool,
    /// Cantidad total de cafe molido consumido
    pub cafe_consumido: u32,
    /// Cantidad total de granos de cafe consumidos
    pub granos_consumidos: u32,
}

impl ContenedorCafe {
    #[must_use]
    pub fn new() -> Self {
        ContenedorCafe {
            cafe_molido: 0,
            granos: G,
            fin: false,
            cafe_consumido: 0,
            granos_consumidos: 0,
        }
    }
}

impl Default for ContenedorCafe {
    fn default() -> Self {
        Self::new()
    }
}

/// Loop donde se rellena el contenedor de cafe molido cuando su cantidad sea menor a [`MAX_CANTIDAD`]
/// y el contenedor se encuentre disponible. Se rellena por completo en [`TIEMPO_CAFE`] milisegundos,
/// durante este tiempo no se podrá utilizar el dispensador de café.
/// También se rellena el contenedor de granos cuando su cantidad sea menor a [`C`], esto es instantáneo.
/// Finaliza cuando [`ContenedorCafe`].fin es true.
///
/// # Errors
/// * En caso de que el lock del contenedor se encuentre envenenado, devuelve [`CafeteriaError::LockEnvenenado`].
pub fn rellenar_cafe(
    contenedor: Arc<(Mutex<ContenedorCafe>, Condvar)>,
) -> Result<(), CafeteriaError> {
    let (cafe_lock, cafe_cvar) = &*contenedor;
    loop {
        if let Ok(mut state) = cafe_cvar.wait_while(cafe_lock.lock()?, |cont| {
            cont.cafe_molido >= MAX_CANTIDAD && !cont.fin
        }) {
            if state.fin {
                break;
            }
            println!("[DEBUG] Reponiendo cafe molido");
            thread::sleep(Duration::from_millis(TIEMPO_CAFE));
            let cantidad = min(C - state.cafe_molido, state.granos);
            state.cafe_molido += cantidad;
            state.granos -= cantidad;
            state.granos_consumidos += cantidad;
            if state.granos < C {
                println!(
                    "[INFO] Contenedor de granos por debajo del {}%. Reponiendo.",
                    C * 100 / G
                );
                state.granos = G;
            }
            cafe_cvar.notify_one();
        }
    }
    Ok(())
}

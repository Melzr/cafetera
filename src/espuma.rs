use std::thread;
use std::sync::{Condvar, Mutex, Arc};
use std::time::Duration;
use std::cmp::min;

use crate::constantes::{E, L, TIEMPO_ESPUMA, MAX_CANTIDAD};
use crate::error::CafeteriaError;

pub struct ContenedorEspuma {
    /// Cantidad actual de espuma
    pub espuma: u32,
    /// Cantidad actual de leche
    pub leche: u32,
    /// true si el contenedor se encuentra en uso.
    /// Solo puede ser usado por un dispensador a la vez
    pub en_uso: bool,
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
            en_uso: false,
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

pub fn rellenar_espuma(contenedor: Arc<(Mutex<ContenedorEspuma>, Condvar)>) -> Result<(), CafeteriaError> {
    let (espuma_lock, espuma_cvar) = &*contenedor;
    loop {
        if let Ok(mut state) = espuma_cvar.wait_while(espuma_lock.lock()?, |cont| {
            (cont.en_uso || cont.espuma >= MAX_CANTIDAD) && !cont.fin
        }) {
            if state.fin {
                break;
            }
            state.en_uso = true;
            println!("[DEBUG] Reponiendo espuma");
            thread::sleep(Duration::from_millis(TIEMPO_ESPUMA));
            let cantidad = min(E - state.espuma, state.leche);
            state.espuma += cantidad;
            state.leche -= cantidad;
            state.leche_consumida += cantidad;
            if state.leche < E {
                println!("[INFO] Contenedor de leche por debajo del {}%. Reponiendo.", E * 100 / L);
                state.leche = L;
            }
            state.en_uso = false;
            espuma_cvar.notify_one();
        }
    }
    Ok(())
}


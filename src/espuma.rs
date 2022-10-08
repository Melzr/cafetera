use std::thread;
use std::sync::{Condvar, Mutex, Arc};
use std::time::Duration;
use std::cmp::min;

use crate::constantes::{E, L, TIEMPO_ESPUMA, MAX_CANTIDAD};

pub struct ContenedorEspuma {
    /// Cantidad actual de espuma
    pub espuma: u32,
    /// Cantidad actual de leche
    pub leche: u32,
    /// true si el contenedor se encuentra en uso.
    /// Solo puede ser usado por un dispensador a la vez
    pub en_uso: bool,
    /// true si no quedan pedidos por realizar
    pub fin: bool
}

impl ContenedorEspuma {
    #[must_use]
    pub fn new() -> Self {
        ContenedorEspuma {
            espuma: 0,
            leche: L,
            en_uso: false,
            fin: false
        }
    }
}

impl Default for ContenedorEspuma {
    fn default() -> Self {
        Self::new()
    }
}

pub fn rellenar_expuma(contenedor: Arc<(Mutex<ContenedorEspuma>, Condvar)>) {
    let (espuma_lock, espuma_cvar) = &*contenedor;
    loop {
        if let Ok(mut state) = espuma_cvar.wait_while(espuma_lock.lock().unwrap(), |cont| {
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
            if state.leche < E {
                println!("[INFO] contenedor de leche por debajo del {}%. Reponiendo.", E * 100 / L);
                state.leche = L;
            }
            state.en_uso = false;
            espuma_cvar.notify_one();
        }
    }
}


use std::thread;
use std::sync::{Condvar, Mutex, Arc};
use std::time::Duration;
use std::cmp::min;

use crate::constantes::{C, G, TIEMPO_CAFE, MAX_CANTIDAD};

pub struct ContenedorCafe {
    /// Cantidad actual de cafe molido
    pub cafe_molido: u32,
    /// Cantidad actual de granos de cafe
    pub granos: u32,
    /// true si el contenedor se encuentra en uso.
    /// Solo puede ser usado por un dispensador a la vez
    pub en_uso: bool,
    /// true si no quedan pedidos por realizar
    pub fin: bool
}

impl ContenedorCafe {
    #[must_use]
    pub fn new() -> Self {
        ContenedorCafe {
            cafe_molido: 0,
            granos: G,
            en_uso: false,
            fin: false
        }
    }
}

impl Default for ContenedorCafe {
    fn default() -> Self {
        Self::new()
    }
}

pub fn rellenar_cafe(contenedor: Arc<(Mutex<ContenedorCafe>, Condvar)>) {
    let (cafe_lock, cafe_cvar) = &*contenedor;
    loop {
        if let Ok(mut state) = cafe_cvar.wait_while(cafe_lock.lock().unwrap(), |cont| {
            (cont.en_uso || cont.cafe_molido >= MAX_CANTIDAD) && !cont.fin
        }) {
            if state.fin {
                break;
            }
            state.en_uso = true;
            println!("[DEBUG] Reponiendo cafe molido");
            thread::sleep(Duration::from_millis(TIEMPO_CAFE));
            let cantidad = min(C - state.cafe_molido, state.granos);
            state.cafe_molido += cantidad;
            state.granos -= cantidad;
            if state.granos < C {
                println!("[INFO] contenedor de granos por debajo del {}%. Reponiendo.", C * 100 / G);
                state.granos = G;
            }
            state.en_uso = false;
            cafe_cvar.notify_one();
        }
    }
}

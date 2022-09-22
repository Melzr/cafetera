use std::sync::{Arc, Mutex, Condvar};

use crate::constantes::{G, L};

pub struct Cafetera {
    pub id: usize,
    /// Contenedor de cafe
    pub cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>,
    /// Contenedor de espuma
    pub espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>,
}

pub struct ContenedorCafe {
    /// Cantidad actual de cafe molido
    pub cafe_molido: u32,
    /// Cantidad actual de granos de cafe
    pub granos: u32,
    /// true si el contenedor se encuentra en uso.
    /// Solo puede ser usado por un dispensador a la vez
    pub en_uso: bool
}

pub struct ContenedorEspuma {
    /// Cantidad actual de espuma
    pub espuma: u32,
    /// Cantidad actual de leche
    pub leche: u32,
    /// true si el contenedor se encuentra en uso.
    /// Solo puede ser usado por un dispensador a la vez
    pub en_uso: bool
}

impl Cafetera {
    #[must_use]
    pub fn new(id: usize) -> Cafetera {
        Cafetera {
            id,
            cafe: Arc::new((
                Mutex::new(ContenedorCafe{cafe_molido: 0, granos: G, en_uso: false}),
                Condvar::new()
            )),
            espuma: Arc::new((
                Mutex::new(ContenedorEspuma{espuma: 0, leche: L, en_uso: false}),
                Condvar::new()
            )),
        }
    }
}

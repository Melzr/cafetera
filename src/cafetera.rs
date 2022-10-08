use std::thread;
use std::sync::{Condvar, Mutex, Arc};
use std::time::Duration;

use crate::constantes::{N, C, E, G, L, TIEMPO_POR_UNIDAD, TIEMPO_CAFE, TIEMPO_ESPUMA, TIEMPO_GRANOS, TIEMPO_LECHE};
use crate::pedido::Pedido;

pub struct Cafetera {
    dispensadores: Arc<(Mutex<Vec<bool>>, Condvar)>,
    /// Contenedor de cafe y granos
    pub cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>,
    /// Contenedor de espuma y leche
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
    pub fn new() -> Cafetera {
        Cafetera {
            dispensadores: Arc::new((Mutex::new(vec![true; N]), Condvar::new())),
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

    pub fn realizar_pedidos(&self, pedidos: Vec<Pedido>) {
        let mut handles = Vec::new();        
        for (i, pedido) in pedidos.into_iter().enumerate() {
            let dispensador = self.obtener_dispensador(i);
            let dispensadores = self.dispensadores.clone();
            let cafe = self.cafe.clone();
            let espuma = self.espuma.clone();
            handles.push(
                thread::spawn(move || {
                    realizar_pedido(i, &pedido, dispensadores, dispensador, cafe, espuma);
                })
            );
        }

        for h in handles {
            h.join().unwrap();
        }
    }
    
    fn obtener_dispensador(&self, pedido: usize) -> usize {
        let (lock, cvar) = &*(self.dispensadores);
        println!("Pedido {} esperando dispensador", pedido);
        let mut num_disp = 0;
        if let Ok(mut state) = cvar.wait_while(lock.lock().unwrap(), |disp| {
            !disp.iter().any(|&x| x)
        }) {
            for (i, disp) in state.iter_mut().enumerate() {
                if *disp {
                    *disp = false;
                    num_disp = i;
                    println!("Pedido {} en dispensador {}", pedido, i);
                    break;
                }
            }
        }
        num_disp
    }

}

impl Default for Cafetera {
    fn default() -> Self {
        Self::new()
    }
}

fn realizar_pedido(id_pedido: usize, pedido: &Pedido, dispensadores: Arc<(Mutex<Vec<bool>>, Condvar)>, dispensador: usize, cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>, espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>) {
    println!("[INFO] Pedido {} sirviendo agua", id_pedido);
    thread::sleep(Duration::from_millis(u64::from(pedido.agua) * TIEMPO_POR_UNIDAD));
    
    let (cafe_lock, cafe_cvar) = &*cafe;
    if let Ok(mut state) = cafe_cvar.wait_while(cafe_lock.lock().unwrap(), |cont| {
        cont.en_uso
    }) {
        state.en_uso = true;
        if pedido.cafe > state.cafe_molido {
            if state.granos < C {
                println!("[INFO] Reponiendo granos");
                thread::sleep(Duration::from_millis(TIEMPO_GRANOS));
                state.granos = G;
            }
            println!("[INFO] Reponiendo cafe molido");
            thread::sleep(Duration::from_millis(TIEMPO_CAFE));
            let cantidad = C - state.cafe_molido;
            state.cafe_molido += cantidad;
            state.granos -= cantidad;
        }
        println!("[INFO] Pedido {} sirviendo cafe", id_pedido);
        thread::sleep(Duration::from_millis(u64::from(pedido.cafe) * TIEMPO_POR_UNIDAD));
        state.cafe_molido -= pedido.cafe;
        println!("[INFO] Cafe: {}", state.cafe_molido);
        state.en_uso = false;
        cafe_cvar.notify_one();
    }

    let (esp_lock, esp_cvar) = &*espuma;
    if let Ok(mut state) = esp_cvar.wait_while(esp_lock.lock().unwrap(), |cont| {
        cont.en_uso
    }) {
        state.en_uso = true;
        if pedido.espuma > state.espuma {
            if state.leche < E {
                println!("[INFO] Reponiendo leche");
                thread::sleep(Duration::from_millis(TIEMPO_LECHE));
                state.leche = L;
            }
            println!("[INFO] Reponiendo espuma");
            thread::sleep(Duration::from_millis(TIEMPO_ESPUMA));
            let cantidad = E - state.espuma;
            state.espuma += cantidad;
            state.leche -= cantidad;
        }
        println!("[INFO] Pedido {} sirviendo espuma", id_pedido);
        thread::sleep(Duration::from_millis(u64::from(pedido.espuma) * TIEMPO_POR_UNIDAD));
        state.espuma -= pedido.espuma;
        println!("[INFO] Espuma: {}", state.espuma);
        state.en_uso = false;
        esp_cvar.notify_one();
    }

    println!("[INFO] Pedido {} completado!", id_pedido);

    let (disp_lock, disp_cvar) = &*dispensadores;
    if let Ok(mut state) = disp_lock.lock() {
        state[dispensador] = true;
    }
    disp_cvar.notify_one();
}

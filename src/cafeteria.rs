use std::thread;
use std::sync::{Condvar, Mutex, Arc};
use std::time::Duration;

use crate::constantes::{CAFETERAS, N, C, E, G, L, TIEMPO_POR_UNIDAD, TIEMPO_CAFE, TIEMPO_ESPUMA, TIEMPO_GRANOS, TIEMPO_LECHE};
use crate::cafetera::{Cafetera, ContenedorCafe, ContenedorEspuma};
use crate::pedido::Pedido;

pub struct Cafeteria {
    cafeteras: Vec<Cafetera>,
    dispensadores: Arc<(Mutex<Vec<bool>>, Condvar)>,
}

impl Cafeteria {
    #[must_use]
    pub fn new() -> Cafeteria {
        Cafeteria {
            cafeteras: (0..CAFETERAS).map(Cafetera::new).collect(),
            dispensadores: Arc::new((Mutex::new(vec![true; N*CAFETERAS]), Condvar::new())),
        }
    }

    pub fn realizar_pedidos(&self, pedidos: Vec<Pedido>) {
        let mut handles = Vec::new();        
        for (i, pedido) in pedidos.into_iter().enumerate() {
            let dispensador = self.obtener_dispensador(i);
            let dispensadores = self.dispensadores.clone();
            let cafetera = &self.cafeteras[dispensador%CAFETERAS];
            let cafe = cafetera.cafe.clone();
            let espuma = cafetera.espuma.clone();
            let id_cafetera = cafetera.id;
            handles.push(
                thread::spawn(move || {
                    realizar_pedido(i, &pedido, dispensadores, dispensador, id_cafetera, cafe, espuma);
                })
            );
        }

        for h in handles {
            h.join().unwrap();
        }
    }
    
    fn obtener_dispensador(&self, id: usize) -> usize {
        let (lock, cvar) = &*(self.dispensadores);
        println!("Pedido {} esperando dispensador", id);
        let mut num_disp = 0;
        if let Ok(mut state) = cvar.wait_while(lock.lock().unwrap(), |disp| {
            !disp.iter().any(|&x| x)
        }) {
            for (i, disp) in state.iter_mut().enumerate() {
                if *disp {
                    *disp = false;
                    num_disp = i;
                    println!("Pedido {} en dispensador {}", id, i);
                    break;
                }
            }
        }
        num_disp
    }

}

impl Default for Cafeteria {
    fn default() -> Self {
        Self::new()
    }
}

fn realizar_pedido(id_pedido: usize, pedido: &Pedido, dispensadores: Arc<(Mutex<Vec<bool>>, Condvar)>, dispensador: usize, id_cafetera: usize, cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>, espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>) {
    println!("[Cafetera {}] Pedido {} sirviendo agua", id_cafetera, id_pedido);
    thread::sleep(Duration::from_millis(u64::from(pedido.agua) * TIEMPO_POR_UNIDAD));
    
    let (cafe_lock, cafe_cvar) = &*cafe;
    if let Ok(mut state) = cafe_cvar.wait_while(cafe_lock.lock().unwrap(), |cont| {
        cont.en_uso
    }) {
        state.en_uso = true;
        if pedido.cafe > state.cafe_molido {
            if state.granos < C {
                println!("[Cafetera {}] Reponiendo granos", id_cafetera);
                thread::sleep(Duration::from_millis(TIEMPO_GRANOS));
                state.granos = G;
            }
            println!("[Cafetera {}] Reponiendo cafe molido", id_cafetera);
            thread::sleep(Duration::from_millis(TIEMPO_CAFE));
            let cantidad = C - state.cafe_molido;
            state.cafe_molido += cantidad;
            state.granos -= cantidad;
        }
        println!("[Cafetera {}] Pedido {} sirviendo cafe", id_cafetera, id_pedido);
        thread::sleep(Duration::from_millis(u64::from(pedido.cafe) * TIEMPO_POR_UNIDAD));
        state.cafe_molido -= pedido.cafe;
        println!("[Cafetera {}] Cafe: {}", id_cafetera, state.cafe_molido);
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
                println!("[Cafetera {}] Reponiendo leche", id_cafetera);
                thread::sleep(Duration::from_millis(TIEMPO_LECHE));
                state.leche = L;
            }
            println!("[Cafetera {}] Reponiendo espuma", id_cafetera);
            thread::sleep(Duration::from_millis(TIEMPO_ESPUMA));
            let cantidad = E - state.espuma;
            state.espuma += cantidad;
            state.leche -= cantidad;
        }
        println!("[Cafetera {}] Pedido {} sirviendo espuma", id_cafetera, id_pedido);
        thread::sleep(Duration::from_millis(u64::from(pedido.espuma) * TIEMPO_POR_UNIDAD));
        state.espuma -= pedido.espuma;
        println!("[Cafetera {}] Espuma: {}", id_cafetera, state.espuma);
        state.en_uso = false;
        esp_cvar.notify_one();
    }

    println!("[Cafetera {}] Pedido {} completado!", id_cafetera, id_pedido);

    let (disp_lock, disp_cvar) = &*dispensadores;
    if let Ok(mut state) = disp_lock.lock() {
        state[dispensador] = true;
    }
    disp_cvar.notify_one();
}

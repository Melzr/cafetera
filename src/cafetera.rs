use std::thread;
use std::sync::{Condvar, Mutex, Arc};
use std::time::Duration;

use crate::constantes::{N, TIEMPO_POR_UNIDAD};
use crate::pedido::Pedido;
use crate::cafe::{ContenedorCafe, rellenar_cafe};
use crate::espuma::{ContenedorEspuma, rellenar_expuma};

pub struct Cafetera {
    dispensadores: Arc<(Mutex<Vec<bool>>, Condvar)>,
    /// Contenedor de cafe y granos
    pub cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>,
    /// Contenedor de espuma y leche
    pub espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>,
}

impl Cafetera {
    #[must_use]
    pub fn new() -> Cafetera {
        Cafetera {
            dispensadores: Arc::new((Mutex::new(vec![true; N]), Condvar::new())),
            cafe: Arc::new((
                Mutex::new(ContenedorCafe::new()),
                Condvar::new()
            )),
            espuma: Arc::new((
                Mutex::new(ContenedorEspuma::new()),
                Condvar::new()
            )),
        }
    }

    pub fn realizar_pedidos(&self, pedidos: Vec<Pedido>) {
        let mut pedidos_handles = Vec::new();   
        let mut contenedores_handles = Vec::new();   
        let cafe = self.cafe.clone();
        contenedores_handles.push(thread::spawn(move || {
            rellenar_cafe(cafe);
        }));
        let espuma = self.espuma.clone();
        contenedores_handles.push(thread::spawn(move || {
            rellenar_expuma(espuma);
        })); 
        for (i, pedido) in pedidos.into_iter().enumerate() {
            let dispensador = self.obtener_dispensador(i);
            let dispensadores = self.dispensadores.clone();
            let cafe = self.cafe.clone();
            let espuma = self.espuma.clone();
            pedidos_handles.push(
                thread::spawn(move || {
                    Self::realizar_pedido(i, &pedido, dispensadores, dispensador, cafe, espuma);
                })
            );
        }

        for h in pedidos_handles {
            h.join().unwrap();
        }

        let (cafe_lock, cafe_cvar) = &*self.cafe;
        cafe_lock.lock().unwrap().fin = true;
        cafe_cvar.notify_all();
        let (espuma_lock, espuma_cvar) = &*self.espuma;
        espuma_lock.lock().unwrap().fin = true;
        espuma_cvar.notify_all();

        for h in contenedores_handles {
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

    fn realizar_pedido(id_pedido: usize, pedido: &Pedido, dispensadores: Arc<(Mutex<Vec<bool>>, Condvar)>, dispensador: usize, cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>, espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>) {
        println!("[DEBUG] Pedido {} sirviendo agua", id_pedido);
        thread::sleep(Duration::from_millis(u64::from(pedido.agua) * TIEMPO_POR_UNIDAD));
        
        Self::servir_cafe(cafe, pedido, id_pedido);
        Self::servir_espuma(espuma, pedido, id_pedido);

        println!("[INFO] Pedido {} completado!", id_pedido);
        let (disp_lock, disp_cvar) = &*dispensadores;
        if let Ok(mut state) = disp_lock.lock() {
            state[dispensador] = true;
        }
        disp_cvar.notify_one();
    }

    fn servir_cafe(contenedor_cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>, pedido: &Pedido, id_pedido: usize) {
        let (cafe_lock, cafe_cvar) = &*contenedor_cafe;
        if let Ok(mut state) = cafe_cvar.wait_while(cafe_lock.lock().unwrap(), |cont| {
            cont.en_uso || cont.cafe_molido < pedido.cafe
        }) {
            state.en_uso = true;
            println!("[DEBUG] Pedido {} sirviendo cafe", id_pedido);
            thread::sleep(Duration::from_millis(u64::from(pedido.cafe) * TIEMPO_POR_UNIDAD));
            state.cafe_molido -= pedido.cafe;
            state.en_uso = false;
            println!("[DEBUG] Pedido {} cafe completado", id_pedido);
            cafe_cvar.notify_all();
        }
    }

    fn servir_espuma(contenedor_espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>, pedido: &Pedido, id_pedido: usize) {
        let (esp_lock, esp_cvar) = &*contenedor_espuma;
        if let Ok(mut state) = esp_cvar.wait_while(esp_lock.lock().unwrap(), |cont| {
            cont.en_uso || cont.espuma < pedido.espuma
        }) {
            state.en_uso = true;
            println!("[DEBUG] Pedido {} sirviendo espuma", id_pedido);
            thread::sleep(Duration::from_millis(u64::from(pedido.espuma) * TIEMPO_POR_UNIDAD));
            state.espuma -= pedido.espuma;
            state.en_uso = false;
            println!("[DEBUG] Pedido {} espuma completada", id_pedido);
            esp_cvar.notify_all();
        }
    }

}

impl Default for Cafetera {
    fn default() -> Self {
        Self::new()
    }
}

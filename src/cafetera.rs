use std::thread::{self, JoinHandle};
use std::sync::{Condvar, Mutex, Arc};
use std::time::Duration;

use crate::constantes::{N, TIEMPO_POR_UNIDAD, TIEMPO_STATS};
use crate::pedido::Pedido;
use crate::cafe::{ContenedorCafe, rellenar_cafe};
use crate::espuma::{ContenedorEspuma, rellenar_expuma};

pub struct Cafetera {
    dispensadores: Arc<(Mutex<Vec<bool>>, Condvar)>,
    /// Contenedor de cafe y granos
    cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>,
    /// Contenedor de espuma y leche
    espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>,
    /// Cantidad total de pedidos completados
    cant_pedidos: Arc<Mutex<u32>>,
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
            cant_pedidos: Arc::new(Mutex::new(0)),
        }
    }

    pub fn realizar_pedidos(&self, pedidos: Vec<Pedido>) {
        let mut pedidos_handles = Vec::new();   
        let mut cafetera_handles = Vec::new();   
        let cafe = self.cafe.clone();
        cafetera_handles.push(thread::spawn(move || {
            rellenar_cafe(cafe);
        }));
        let espuma = self.espuma.clone();
        cafetera_handles.push(thread::spawn(move || {
            rellenar_expuma(espuma);
        }));
        cafetera_handles.push(self.estadisticas());
        for (i, pedido) in pedidos.into_iter().enumerate() {
            let dispensador = self.obtener_dispensador(i);
            pedidos_handles.push(self.realizar_pedido(i, pedido, dispensador));
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

        for h in cafetera_handles {
            h.join().unwrap();
        }

    }

    fn obtener_dispensador(&self, pedido: usize) -> usize {
        let (lock, cvar) = &*(self.dispensadores);
        println!("[DEBUG] Pedido {} esperando dispensador", pedido);
        let mut num_disp = 0;
        if let Ok(mut state) = cvar.wait_while(lock.lock().unwrap(), |disp| {
            !disp.iter().any(|&x| x)
        }) {
            for (i, disp) in state.iter_mut().enumerate() {
                if *disp {
                    *disp = false;
                    num_disp = i;
                    println!("[DEBUG] Pedido {} en dispensador {}", pedido, i);
                    break;
                }
            }
        }
        num_disp
    }

    fn realizar_pedido(&self, id_pedido: usize, pedido: Pedido, dispensador: usize) -> JoinHandle<()> {
        let dispensadores = self.dispensadores.clone();
        let cafe = self.cafe.clone();
        let espuma = self.espuma.clone();
        let pedidos_lock = self.cant_pedidos.clone();

        thread::spawn(move || {
            println!("[DEBUG] Pedido {} sirviendo agua", id_pedido);
            thread::sleep(Duration::from_millis(u64::from(pedido.agua) * TIEMPO_POR_UNIDAD));
            
            Self::servir_cafe(cafe, &pedido, id_pedido);
            Self::servir_espuma(espuma, &pedido, id_pedido);
    
            println!("[INFO] Pedido {} completado!", id_pedido);
            let (disp_lock, disp_cvar) = &*dispensadores;
            if let Ok(mut state) = disp_lock.lock() {
                state[dispensador] = true;
            }
            if let Ok(mut cant_pedidos) = pedidos_lock.lock() {
                *cant_pedidos += 1;
            }
            disp_cvar.notify_one();
        })
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
            state.cafe_consumido += pedido.cafe;
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
            state.espuma_consumida += pedido.espuma;
            state.en_uso = false;
            println!("[DEBUG] Pedido {} espuma completada", id_pedido);
            esp_cvar.notify_all();
        }
    }

    fn estadisticas(&self) -> JoinHandle<()> {
        let cafe_lock = self.cafe.clone();
        let espuma_lock = self.espuma.clone();
        let pedidos_lock = self.cant_pedidos.clone();

        thread::spawn(move || loop {
            let (mut cant_cafe, mut cant_granos, mut cant_espuma, mut cant_leche) = (0, 0, 0, 0);    
            let (mut cafe_cons, mut granos_cons, mut espuma_cons, mut leche_cons) = (0, 0, 0, 0);    
            let mut cant_pedidos = 0;

            if let Ok(cafe_contenedor) = cafe_lock.0.lock() {
                cant_cafe = cafe_contenedor.cafe_molido;
                cant_granos = cafe_contenedor.granos;
                cafe_cons = cafe_contenedor.cafe_consumido;
                granos_cons = cafe_contenedor.granos_consumidos;
            }
            if let Ok(espuma_contenedor) = espuma_lock.0.lock() {
                cant_espuma = espuma_contenedor.espuma;
                cant_leche = espuma_contenedor.leche;
                espuma_cons = espuma_contenedor.espuma_consumida;
                leche_cons = espuma_contenedor.leche_consumida;
            }
            if let Ok(pedidos) = pedidos_lock.lock() {
                cant_pedidos = *pedidos;
            }

            println!("[INFO] Estado contenedores: cafe {}, granos {}, espuma {}, leche {}", cant_cafe, cant_granos, cant_espuma, cant_leche);
            println!("[INFO] Consumo total: cafe {}, granos {}, espuma {}, leche {}", cafe_cons, granos_cons, espuma_cons, leche_cons);
            println!("[INFO] Pedidos completados: {}", cant_pedidos);
            
            match espuma_lock.0.lock() {
                Ok(contenedor_espuma) => {
                    if contenedor_espuma.fin {
                        break;
                    }
                },
                Err(_) => {
                    println!("[ERROR] Debido a un error inesperado no se seguiran mostrando las estadisticas");
                    break;
                },
            }

            thread::sleep(Duration::from_millis(TIEMPO_STATS));
        })


    }
}

impl Default for Cafetera {
    fn default() -> Self {
        Self::new()
    }
}

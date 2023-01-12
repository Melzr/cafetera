use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::cafe::{rellenar_cafe, ContenedorCafe};
use crate::constantes::{N, TIEMPO_PEDIDO, TIEMPO_POR_UNIDAD, TIEMPO_STATS};
use crate::error::CafeteriaError;
use crate::espuma::{rellenar_espuma, ContenedorEspuma};
use crate::pedido::Pedido;

pub struct Cafetera {
    dispensadores: Arc<(Mutex<Vec<bool>>, Condvar)>,
    /// Contenedor de cafe y granos
    pub cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>,
    /// Contenedor de espuma y leche
    pub espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>,
    /// Cantidad total de pedidos completados
    pub cant_pedidos: Arc<Mutex<u32>>,
}

impl Cafetera {
    #[must_use]
    pub fn new() -> Cafetera {
        Cafetera {
            dispensadores: Arc::new((Mutex::new(vec![true; N]), Condvar::new())),
            cafe: Arc::new((Mutex::new(ContenedorCafe::new()), Condvar::new())),
            espuma: Arc::new((Mutex::new(ContenedorEspuma::new()), Condvar::new())),
            cant_pedidos: Arc::new(Mutex::new(0)),
        }
    }

    /// Lee el archivo de pedidos dado por el argumento ruta y los prepara.
    ///
    /// # Errors
    /// * En caso de error al abrir el archivo, devuelve [`CafeteriaError::AperturaArchivo`].
    /// * En caso de error al leer el archivo, devuelve [`CafeteriaError::LecturaArchivo`].
    /// * En caso de que el lock de los dispensadores se encuentre envenenado, devuelve [`CafeteriaError::LockEnvenenado`].
    pub fn realizar_pedidos(&self, ruta: &str) -> Result<(), CafeteriaError> {
        let file = File::open(ruta).map_err(|_| CafeteriaError::AperturaArchivo)?;
        let file = BufReader::new(file);
        let mut pedidos_handles = Vec::new();
        let mut cafetera_handles = Vec::new();

        let cafe = self.cafe.clone();
        cafetera_handles.push(thread::spawn(move || {
            if rellenar_cafe(cafe).is_err() {
                println!("[ERROR] No se pudo rellenar cafe");
            }
        }));

        let espuma = self.espuma.clone();
        cafetera_handles.push(thread::spawn(move || {
            if rellenar_espuma(espuma).is_err() {
                println!("[ERROR] No se pudo rellenar espuma");
            }
        }));

        cafetera_handles.push(self.estadisticas());

        for line in file.lines() {
            let line = line.map_err(|_| CafeteriaError::LecturaArchivo)?;
            match Pedido::from_line(&line) {
                Ok(pedido) => {
                    let dispensador = self.obtener_dispensador(pedido.id)?;
                    pedidos_handles.push(self.realizar_pedido(pedido, dispensador));
                    thread::sleep(Duration::from_millis(TIEMPO_PEDIDO));
                }
                Err(e) => {
                    println!("[WARN] Error al procesar el pedido: {:?}", e);
                }
            }
        }

        for h in pedidos_handles {
            if h.join().is_err() {
                println!("[WARN] Error en el join de un hilo");
            }
        }

        let (cafe_lock, cafe_cvar) = &*self.cafe;
        cafe_lock.lock()?.fin = true;
        cafe_cvar.notify_all();
        let (espuma_lock, espuma_cvar) = &*self.espuma;
        espuma_lock.lock()?.fin = true;
        espuma_cvar.notify_all();

        for h in cafetera_handles {
            if h.join().is_err() {
                println!("[WARN] Error en el join de un hilo");
            }
        }

        Ok(())
    }

    /// Obtiene un dispensador libre para el pedido.
    ///
    /// # Errors
    /// * En caso de que el lock de los dispensadores se encuentre envenenado, devuelve [`CafeteriaError::LockEnvenenado`].
    fn obtener_dispensador(&self, pedido: usize) -> Result<usize, CafeteriaError> {
        let (lock, cvar) = &*(self.dispensadores);
        println!("[DEBUG] Pedido {} esperando dispensador", pedido);
        let mut num_disp = 0;
        if let Ok(mut state) = cvar.wait_while(lock.lock()?, |disp| !disp.iter().any(|&x| x)) {
            for (i, disp) in state.iter_mut().enumerate() {
                if *disp {
                    *disp = false;
                    num_disp = i;
                    println!("[DEBUG] Pedido {} en dispensador {}", pedido, i);
                    break;
                }
            }
        }
        Ok(num_disp)
    }

    /// Realiza el pedido utilizando el dispensador recibido en un thread aparte,
    /// devolviendo su correspondiente [`JoinHandle`].
    fn realizar_pedido(&self, pedido: Pedido, dispensador: usize) -> JoinHandle<()> {
        let dispensadores = self.dispensadores.clone();
        let cafe = self.cafe.clone();
        let espuma = self.espuma.clone();
        let pedidos_lock = self.cant_pedidos.clone();

        thread::spawn(move || {
            println!("[DEBUG] Pedido {} sirviendo agua", pedido.id);
            thread::sleep(Duration::from_millis(
                u64::from(pedido.agua) * TIEMPO_POR_UNIDAD,
            ));

            if Self::servir_cafe(cafe, &pedido).is_err() {
                println!("[WARN] Pedido {} no pudo servir cafe", pedido.id);
            }
            if Self::servir_espuma(espuma, &pedido).is_err() {
                println!("[WARN] Pedido {} no pudo servir espuma", pedido.id);
            }

            println!("[INFO] Pedido {} completado!", pedido.id);
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

    /// Sirve cafe al pedido recibido.
    ///
    /// # Errors
    /// * En caso de que el lock de los dispensadores se encuentre envenenado, devuelve [`CafeteriaError::LockEnvenenado`].
    fn servir_cafe(
        contenedor_cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>,
        pedido: &Pedido,
    ) -> Result<(), CafeteriaError> {
        let (cafe_lock, cafe_cvar) = &*contenedor_cafe;
        if let Ok(mut state) = cafe_cvar.wait_while(cafe_lock.lock()?, |cont| {
            cont.cafe_molido < pedido.cafe
        }) {
            println!("[DEBUG] Pedido {} sirviendo cafe", pedido.id);
            thread::sleep(Duration::from_millis(
                u64::from(pedido.cafe) * TIEMPO_POR_UNIDAD,
            ));
            state.cafe_molido -= pedido.cafe;
            state.cafe_consumido += pedido.cafe;
            println!("[DEBUG] Pedido {} cafe completado", pedido.id);
            cafe_cvar.notify_all();
        }
        Ok(())
    }

    /// Sirve espuma al pedido recibido.
    ///
    /// # Errors
    /// * En caso de que el lock de los dispensadores se encuentre envenenado, devuelve [`CafeteriaError::LockEnvenenado`].
    fn servir_espuma(
        contenedor_espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>,
        pedido: &Pedido,
    ) -> Result<(), CafeteriaError> {
        let (esp_lock, esp_cvar) = &*contenedor_espuma;
        if let Ok(mut state) = esp_cvar.wait_while(esp_lock.lock()?, |cont| {
            cont.espuma < pedido.espuma
        }) {
            println!("[DEBUG] Pedido {} sirviendo espuma", pedido.id);
            thread::sleep(Duration::from_millis(
                u64::from(pedido.espuma) * TIEMPO_POR_UNIDAD,
            ));
            state.espuma -= pedido.espuma;
            state.espuma_consumida += pedido.espuma;
            println!("[DEBUG] Pedido {} espuma completada", pedido.id);
            esp_cvar.notify_all();
        }
        Ok(())
    }

    /// Imprime por consola el estado de la cafetera cada [`TIEMPO_STATS`] milisegundos en un hilo
    /// aparte, devolviendo su correspondiente [`JoinHandle`].
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

            println!(
                "[INFO] Estado contenedores: cafe {}, granos {}, espuma {}, leche {}",
                cant_cafe, cant_granos, cant_espuma, cant_leche
            );
            println!(
                "[INFO] Consumo total: cafe {}, granos {}, espuma {}, leche {}",
                cafe_cons, granos_cons, espuma_cons, leche_cons
            );
            println!("[INFO] Pedidos completados: {}", cant_pedidos);

            if let Ok(contenedor_espuma) = espuma_lock.0.lock() {
                if contenedor_espuma.fin {
                    break;
                }
            } else {
                println!("[ERROR] Debido a un error inesperado no se seguiran mostrando las estadisticas");
                break;
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

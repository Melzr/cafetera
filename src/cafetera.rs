use std::sync::{Arc, Mutex, Condvar};
// use std::thread::{self, JoinHandle};
// use std::time::Duration;
// use crate::pedido::Pedido;

// const G: u32 = 1000;
// const C: u32 = 100;
// const L: u32 = 1000;
// const E: u32 = 100;
// const N: usize = 4;

// const TIEMPO_CAFE: u64 = 4000;
// const TIEMPO_GRANOS: u64 = 2000;
// const TIEMPO_ESPUMA: u64 = 4000;
// const TIEMPO_LECHE: u64 = 2000;
// const TIEMPO_POR_UNIDAD: u64 = 200;

pub struct Cafetera {
    pub id: usize,
    pub cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>,
    pub espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>,
}

pub struct ContenedorCafe {
    pub cafe_molido: u32,
    pub en_uso: bool
}

pub struct ContenedorEspuma {
    pub espuma: u32,
    pub en_uso: bool
}

impl Cafetera {
    #[must_use]
    pub fn new(id: usize) -> Cafetera {
        Cafetera {
            id,
            cafe: Arc::new((
                Mutex::new(ContenedorCafe{cafe_molido: 0, en_uso: false}),
                Condvar::new()
            )),
            espuma: Arc::new((
                Mutex::new(ContenedorEspuma{espuma: 0, en_uso: false}),
                Condvar::new()
            )),
        }
    }

    // pub fn producir_cafe(&mut self) -> JoinHandle<()> {
    //     let mut contenedor_granos = G;
    //     let contenedor_cafe = self.cafe.clone();
    //     let id_local = self.id;

    //     thread::spawn(move || loop {
    //         {
    //             let (lock, cvar) = &*contenedor_cafe;
    //             let mut state = cvar.wait_while(lock.lock().unwrap(), |cont| {
    //                 cont.cafe_molido != 0 || cont.en_uso
    //             }).unwrap();
    //             state.en_uso = true;
    //             let cantidad = std::cmp::min(C, contenedor_granos);
    //             println!("[Cafetera {}] Reponiendo cafe molido", id_local);
    //             thread::sleep(Duration::from_millis(TIEMPO_CAFE));
    //             state.cafe_molido += cantidad;
    //             contenedor_granos -= cantidad;
    //             println!("[Cafetera {}] Cafe: {}, granos: {}", id_local, state.cafe_molido, contenedor_granos);
    //             state.en_uso = false;
    //             cvar.notify_all();
    //         }
    //         if contenedor_granos == 0 {
    //             println!("[Cafetera {}] Reponiendo granos", id_local);
    //             thread::sleep(Duration::from_millis(TIEMPO_GRANOS));
    //             contenedor_granos = G;
    //         }
    //     })
    // }

    // pub fn producir_espuma(&mut self) -> JoinHandle<()> {
    //     let mut contenedor_leche = L;
    //     let contenedor_espuma = self.espuma.clone();
    //     let id_local = self.id;

    //     thread::spawn(move || loop {
    //         {
    //             let (lock, cvar) = &*contenedor_espuma;
    //             let mut state = cvar.wait_while(lock.lock().unwrap(), |cont| {
    //                 cont.espuma != 0 || cont.en_uso
    //             }).unwrap();
    //             state.en_uso = true;
    //             let cantidad = std::cmp::min(E, contenedor_leche);
    //             println!("[Cafetera {}] Reponiendo espuma", id_local);
    //             thread::sleep(Duration::from_millis(TIEMPO_ESPUMA));
    //             state.espuma += cantidad;
    //             contenedor_leche -= cantidad;
    //             state.en_uso = false;
    //             println!("[Cafetera {}] Espuma: {}, leche: {}", id_local, state.espuma, contenedor_leche);
    //             cvar.notify_all();
    //         }
    //         if contenedor_leche == 0 {
    //             println!("[Cafetera {}] Reponiendo leche", id_local);
    //             thread::sleep(Duration::from_millis(TIEMPO_LECHE));
    //             contenedor_leche = L;
    //         }
    //     })
    // }

    // pub fn servir(&self, id_pedido: &usize, pedido: Pedido, dispensadores: Arc<(Mutex<Vec<bool>>, Condvar)>, dispensador: usize) -> JoinHandle<()> {
    //     let id_local = self.id;
    //     let id_pedido = *id_pedido;

    //     thread::spawn(|| {
    //         println!("[Cafetera {}] Pedido {} sirviendo agua", id_local, id_pedido);
    //         thread::sleep(Duration::from_millis(u64::from(pedido.agua) * TIEMPO_POR_UNIDAD));
            
    //         let (cafe_lock, cafe_cvar) = &*(self.cafe);
    //         if let Ok(mut state) = cafe_cvar.wait_while(cafe_lock.lock().unwrap(), |cont| {
    //             cont.en_uso
    //         }) {
    //             state.en_uso = true;
    //             if pedido.cafe > state.cafe_molido {
    //                 println!("[Cafetera {}] Reponiendo cafe molido", id_local);
    //                 thread::sleep(Duration::from_millis(TIEMPO_CAFE));
    //                 state.cafe_molido += C;
    //             }
    //             println!("[Cafetera {}] Pedido {} sirviendo cafe", id_local, id_pedido);
    //             thread::sleep(Duration::from_millis(u64::from(pedido.cafe) * TIEMPO_POR_UNIDAD));
    //             state.cafe_molido -= pedido.cafe;
    //             println!("[Cafetera {}] Cafe: {}", id_local, state.cafe_molido);
    //             state.en_uso = false;
    //             cafe_cvar.notify_one();
    //         }
    
    //         let (esp_lock, esp_cvar) = &*(self.espuma);
    //         if let Ok(mut state) = esp_cvar.wait_while(esp_lock.lock().unwrap(), |cont| {
    //             cont.en_uso
    //         }) {
    //             state.en_uso = true;
    //             if pedido.espuma > state.espuma {
    //                 println!("[Cafetera {}] Reponiendo espuma", id_local);
    //                 thread::sleep(Duration::from_millis(TIEMPO_ESPUMA));
    //                 state.espuma += E;
    //             }
    //             println!("[Cafetera {}] Pedido {} sirviendo espuma", id_local, id_pedido);
    //             thread::sleep(Duration::from_millis(u64::from(pedido.espuma) * TIEMPO_POR_UNIDAD));
    //             state.espuma -= pedido.espuma;
    //             println!("[Cafetera {}] Espuma: {}", id_local, state.espuma);
    //             state.en_uso = false;
    //             esp_cvar.notify_one();
    //         }
    
    //         println!("[Cafetera {}] Pedido {} completado!", id_local, id_pedido);
    
    //         let (disp_lock, disp_cvar) = &*dispensadores;
    //         if let Ok(mut state) = disp_lock.lock() {
    //             state[dispensador] = true;
    //         }
    //         disp_cvar.notify_one();
    //     })
    // }
}

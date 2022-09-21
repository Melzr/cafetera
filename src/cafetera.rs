use std::sync::{Arc, Mutex, Condvar};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const G: u32 = 1000;
const C: u32 = 100;
const L: u32 = 1000;
const E: u32 = 100;
const N: usize = 4;

const TIEMPO_CAFE: u64 = 4000;
const TIEMPO_GRANOS: u64 = 2000;
const TIEMPO_ESPUMA: u64 = 4000;
const TIEMPO_LECHE: u64 = 2000;

pub struct Cafetera {
    id: usize,
    cafe: Arc<(Mutex<ContenedorCafe>, Condvar)>,
    espuma: Arc<(Mutex<ContenedorEspuma>, Condvar)>,
    dispensadores: Arc<(Mutex<Vec<bool>>, Condvar)>
}

pub struct ContenedorCafe {
    cafe_molido: u32,
    en_uso: bool
}

pub struct ContenedorEspuma {
    espuma: u32,
    en_uso: bool
}

impl Cafetera {
    #[must_use] pub fn new(id: usize) -> Cafetera {
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
            dispensadores: Arc::new((Mutex::new(vec![true; N]), Condvar::new()))
        }
    }

    pub fn producir_cafe(&mut self) -> JoinHandle<()> {
        let mut contenedor_granos = G;
        let contenedor_cafe = self.cafe.clone();
        let id_local = self.id;

        thread::spawn(move || loop {
            {
                let (lock, cvar) = &*contenedor_cafe;
                let mut state = cvar.wait_while(lock.lock().unwrap(), |cont| {
                    cont.cafe_molido != 0 || cont.en_uso
                }).unwrap();
                state.en_uso = true;
                let cantidad = std::cmp::min(C, contenedor_granos);
                println!("[Cafetera {}] Reponiendo cafe molido", id_local);
                thread::sleep(Duration::from_millis(TIEMPO_CAFE));
                state.cafe_molido += cantidad;
                contenedor_granos -= cantidad;
                println!("[Cafetera {}] Cafe: {}, granos: {}", id_local, state.cafe_molido, contenedor_granos);
                state.en_uso = false;
                cvar.notify_all();
            }
            if contenedor_granos == 0 {
                println!("[Cafetera {}] Reponiendo granos", id_local);
                thread::sleep(Duration::from_millis(TIEMPO_GRANOS));
                contenedor_granos = G;
            }
        })
    }

    pub fn producir_espuma(&mut self) -> JoinHandle<()> {
        let mut contenedor_leche = L;
        let contenedor_espuma = self.espuma.clone();
        let id_local = self.id;

        thread::spawn(move || loop {
            {
                let (lock, cvar) = &*contenedor_espuma;
                let mut state = cvar.wait_while(lock.lock().unwrap(), |cont| {
                    cont.espuma != 0 || cont.en_uso
                }).unwrap();
                state.en_uso = true;
                let cantidad = std::cmp::min(E, contenedor_leche);
                println!("[Cafetera {}] Reponiendo espuma", id_local);
                thread::sleep(Duration::from_millis(TIEMPO_ESPUMA));
                state.espuma += cantidad;
                contenedor_leche -= cantidad;
                state.en_uso = false;
                println!("[Cafetera {}] Espuma: {}, leche: {}", id_local, state.espuma, contenedor_leche);
                cvar.notify_all();
            }
            if contenedor_leche == 0 {
                println!("[Cafetera {}] Reponiendo leche", id_local);
                thread::sleep(Duration::from_millis(TIEMPO_LECHE));
                contenedor_leche = L;
            }
        })
    }

    pub fn servir(&mut self, n: usize, cant_cafe: u32, cant_espuma: u32, cant_agua: u32) -> JoinHandle<()> {
        let dispensadores = self.dispensadores.clone();
        let contenedor_cafe = self.cafe.clone();
        let contenedor_espuma = self.espuma.clone();
        let id_local = self.id;
        
        thread::spawn(move || {
            let (lock, cvar) = &*dispensadores;
            let mut dispensador = 0;
            let mut cafe_servido = 0;
            let mut espuma_servida = 0;

            {
                let mut state = cvar.wait_while(lock.lock().unwrap(), |disp| {
                    println!("[Cafetera {}] Pedido {} esperando dispensador", id_local, n);
                    !disp.iter().any(|&x| x)
                }).unwrap();
                for ing in &mut (*state) {
                    if *ing {
                        println!("[Cafetera {}] Pedido {} en dispensador {}", id_local, n, dispensador);
                        *ing = false;
                        break;
                    }
                    dispensador+=1;
                }
            }

            println!("[Cafetera {}] Pedido {} sirviendo agua", id_local, n);
            thread::sleep(Duration::from_millis(u64::from(cant_agua * 100)));
            
            let (cafe_lock, cafe_cvar) = &*contenedor_cafe;
            while cafe_servido < cant_cafe {
                let mut state = cafe_cvar.wait_while(cafe_lock.lock().unwrap(), |cont| {
                    cont.cafe_molido == 0 || cont.en_uso
                }).unwrap();
                state.en_uso = true;
                let cantidad_a_servir = std::cmp::min(cant_cafe - cafe_servido, state.cafe_molido);
                println!("[Cafetera {}] Pedido {} sirviendo cafe", id_local, n);
                thread::sleep(Duration::from_millis(u64::from(cantidad_a_servir * 100)));
                cafe_servido += cantidad_a_servir;
                state.cafe_molido -= cantidad_a_servir;
                println!("[Cafetera {}] Cafe: {}", id_local, state.cafe_molido);
                state.en_uso = false;
                cafe_cvar.notify_all();
            }

            let (esp_lock, esp_cvar) = &*contenedor_espuma;
            while espuma_servida < cant_espuma {
                let mut state = esp_cvar.wait_while(esp_lock.lock().unwrap(), |cont| {
                    cont.espuma == 0 || cont.en_uso
                }).unwrap();
                state.en_uso = true;
                let cantidad_a_servir = std::cmp::min(cant_espuma - espuma_servida, state.espuma);
                println!("[Cafetera {}] Pedido {}: sirviendo espuma", id_local, n);
                thread::sleep(Duration::from_millis(u64::from(cantidad_a_servir * 100)));
                espuma_servida += cantidad_a_servir;
                state.espuma -= cantidad_a_servir;
                println!("[Cafetera {}] Espuma: {}", id_local, state.espuma);
                state.en_uso = false;
                esp_cvar.notify_all();
            }

            println!("[Cafetera {}] Pedido {} completado!", id_local, n);
            let mut state = lock.lock().unwrap();
            state[dispensador] = true;
            cvar.notify_one();
        })
    }

    pub fn run(&mut self) {
        let mut handlers = vec![self.producir_cafe(), self.producir_espuma()];
        for i in 0..50 {
            handlers.push(self.servir(i, 10, 10, 10));
        }
        for h in handlers {
            h.join().unwrap();
        }
    }

}

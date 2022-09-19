use std::sync::{Arc, Mutex, Condvar};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const G: i32 = 100;
const C: i32 = 100;
const L: i32 = 100;
const E: i32 = 100;
const N: usize = 6;

pub struct Cafetera {
    granos: i32,
    cafe: i32,
    leche: i32,
    espuma: i32,
    dispensadores: Arc<(Mutex<Vec<bool>>, Condvar)>
}

impl Cafetera {
    pub fn new() -> Cafetera {
        Cafetera {
            granos: G,
            cafe: 0,
            leche: L,
            espuma: 0,
            dispensadores: Arc::new((Mutex::new(vec![true; N]), Condvar::new()))
        }
    }

    fn llenar_granos(&mut self) {
        thread::sleep(Duration::from_millis(3000));
        self.print_info();
        self.granos = G;
        println!("Recargue granos");
        self.print_info();
    }

    fn llenar_cafe(&mut self) {
        thread::sleep(Duration::from_millis(3000));
        self.print_info();
        self.cafe = self.granos;
        self.granos = 0;
        println!("Prepare cafe molido");
        self.print_info();
    }

    fn llenar_leche(&mut self) {
        thread::sleep(Duration::from_millis(3000));
        self.print_info();
        self.leche = L;
        println!("Recargue leche");
        self.print_info();
    }

    fn llenar_espuma(&mut self) {
        thread::sleep(Duration::from_millis(3000));
        self.print_info();
        self.espuma = self.leche;
        self.leche = 0;
        println!("Prepare espuma");
        self.print_info();
    }

    fn servir(&mut self, n: usize) -> JoinHandle<()> {
        let dispensadores = self.dispensadores.clone();

        thread::spawn(move || {
            let (lock, cvar) = &*dispensadores;

            let mut state = cvar.wait_while(lock.lock().unwrap(), |disp| {
                println!("Esperando dispensador");
                !disp.iter().any(|&x| x)
            }).unwrap();
            let mut i = 1;
            println!("{:?}", state);
            for ing in (*state).iter_mut() {
                if (*ing) {
                    println!("Entre en dispensador {}", i);
                    *ing = false;
                    break;
                }
                i+=1;
            }
            drop(state);
            thread::sleep(Duration::from_millis(5000));
            println!("Café servido! {}", n);
            let mut state = lock.lock().unwrap();
            state[i-1] = true;
            cvar.notify_one();
        })
    }

    fn print_info(&self) {
        println!("[INFO] Granos: {}, Cafe: {}, Espuma: {}, Leche: {}", self.granos, self.cafe, self.espuma, self.leche);
    }

    pub fn run(&mut self) {
        let mut handlers = Vec::new();
        let mut i = 0;
        for i in 0..12 {
            if self.cafe >= 10 && self.espuma >= 10 {
                handlers.push(self.servir(i));
            } else {
                if self.granos >= 10 {
                    self.llenar_cafe();
                } else {
                    self.llenar_granos();
                }
                if self.leche >= 10 {
                    self.llenar_espuma();
                } else {
                    self.llenar_leche();
                }
            }
        }
        for h in handlers {
            h.join().unwrap();
        }
    }

}

impl Default for Cafetera {
    fn default() -> Self {
        Self::new()
    }
}

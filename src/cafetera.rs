use std::thread;
use std::time::Duration;

const G: i32 = 100;
const C: i32 = 100;
const L: i32 = 100;
const E: i32 = 100;
const N: i32 = 6;

pub struct Cafetera {
    granos: i32,
    cafe: i32,
    leche: i32,
    espuma: i32,
}

impl Cafetera {
    pub fn new() -> Cafetera {
        Cafetera {
            granos: G,
            cafe: 0,
            leche: L,
            espuma: 0,
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

    fn servir(&mut self) {
        thread::sleep(Duration::from_millis(5000));
        self.cafe -= 10;
        self.espuma -= 10;
        // agua
        println!("Café servido!");
        self.print_info();
    }

    fn print_info(&self) {
        println!("[INFO] Granos: {}, Cafe: {}, Espuma: {}, Leche: {}", self.granos, self.cafe, self.espuma, self.leche);
    }

    pub fn run(&mut self) {
        loop {
            if self.cafe >= 10 && self.espuma >= 10 {
                self.servir();
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
    }

}

impl Default for Cafetera {
    fn default() -> Self {
        Self::new()
    }
}

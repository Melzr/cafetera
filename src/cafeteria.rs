use crate::cafetera::Cafetera;

const N: usize = 3;

pub struct Cafeteria {
  cafeteras: Vec<Cafetera>,
}

impl Cafeteria {
  pub fn new() -> Cafeteria {
    Cafeteria {
      cafeteras: (0..N).map(|i| Cafetera::new(i)).collect(),
    }
  }

  pub fn atender_clientes(&mut self) {
    let mut handles = Vec::new();
    for cafetera in self.cafeteras.iter_mut() {
      handles.push(cafetera.producir_cafe());
      handles.push(cafetera.producir_espuma());
    }

    for i in 0..100 {
      let cafetera = &mut self.cafeteras[i % N];
      handles.push(cafetera.servir(i, 10, 10, 10));
    }

    for h in handles {
      h.join().unwrap();
    }
  }
}

use crate::cafetera::Cafetera;
use crate::pedido::Pedido;

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

  pub fn atender_clientes(&mut self, pedidos: Vec<Pedido>) {
    let mut handles = Vec::new();
    for cafetera in self.cafeteras.iter_mut() {
      handles.push(cafetera.producir_cafe());
      handles.push(cafetera.producir_espuma());
    }

    // iterate pedidos and get index
    let mut i = 0;
    for pedido in pedidos {
      let cafetera_index = i % N;
      let cafetera = &mut self.cafeteras[cafetera_index];
      handles.push(cafetera.servir(i, pedido));
      i += 1;
    }

    for h in handles {
      h.join().unwrap();
    }
  }
}

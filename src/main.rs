use std::fs;
use coffee::cafeteria::Cafeteria;
use coffee::pedido::Pedido;
use rand::Rng;

fn main() {
    let data = fs::read_to_string("./pedidos.json")
    .expect("Unable to read file");
    let pedidos: Vec<Pedido> = serde_json::from_str::<Vec<Pedido>>(&data).unwrap();
    
    let mut cafeteria = Cafeteria::new();
    cafeteria.atender_clientes(pedidos);

    // generate 100 random Pedido
    // let mut rng = rand::thread_rng();
    // let pedidos: Vec<Pedido> = (0..100)
    //     .map(|_| Pedido {
    //         agua: rng.gen_range(1u32..=10u32),
    //         cafe: rng.gen_range(1u32..=10u32),
    //         espuma: rng.gen_range(1u32..=10u32),
    //     })
    //     .collect();
    
    // println!("{:?}", pedidos);

    // serialize pedidos to json file
    // let json = serde_json::to_string(&pedidos).unwrap();
    // fs::write("./pedidos.json", json).expect("Unable to write file");

}

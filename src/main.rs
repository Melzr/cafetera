use cafeteria::cafetera::Cafetera;
use cafeteria::error::CafeteriaError;
// use cafeteria::pedido::generate_file;

fn main() -> Result<(), CafeteriaError> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(ruta) => {
            let cafetera = Cafetera::new();
            cafetera.realizar_pedidos(ruta)

            // generate_file(ruta)
        }
        None => {
            println!("No se especificó la ruta del archivo de pedidos");
            Err(CafeteriaError::ArgumentosInvalidos)
        }
    }
}

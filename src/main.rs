use cafeteria::cafetera::Cafetera;
use cafeteria::error::CafeteriaError;
use cafeteria::pedido::generate_file;

const FILE_COMMAND: &str = "-f";

fn main() -> Result<(), CafeteriaError> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(arg) => {
            if arg == FILE_COMMAND {
                let filename = args.get(2).ok_or(CafeteriaError::ArgumentosInvalidos)?;
                let n = args.get(3);
                generate_file(filename, n)
            } else {
                let cafetera = Cafetera::new();
                cafetera.realizar_pedidos(arg)
            }
        }
        None => {
            println!("No se especificó la ruta del archivo de pedidos");
            Err(CafeteriaError::ArgumentosInvalidos)
        }
    }
}

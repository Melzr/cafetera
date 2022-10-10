#[cfg(test)]
mod tests {
    use cafeteria::cafetera::Cafetera;
    use cafeteria::constantes::{C, E, G, L};
    use cafeteria::error::CafeteriaError;

    #[test]
    fn test01_ruta_invalida() {
        let cafetera = Cafetera::new();
        let res = cafetera.realizar_pedidos("");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), CafeteriaError::AperturaArchivo);
    }

    #[test]
    fn test02_cero_pedidos() {
        let cafetera = Cafetera::new();
        let res = cafetera.realizar_pedidos("tests/test02.txt");
        let cant_pedidos = *cafetera.cant_pedidos.lock().unwrap();
        assert!(res.is_ok());
        assert_eq!(cant_pedidos, 0);
    }

    #[test]
    fn test03_un_pedido() {
        let cafetera = Cafetera::new();
        let res = cafetera.realizar_pedidos("tests/test03.txt");
        let cant_pedidos = *cafetera.cant_pedidos.lock().unwrap();
        let contenedor_cafe = cafetera.cafe.0.lock().unwrap();
        let contenedor_espuma = cafetera.espuma.0.lock().unwrap();
        assert!(res.is_ok());
        assert_eq!(cant_pedidos, 1);
        assert_eq!(contenedor_cafe.cafe_consumido, 5);
        assert_eq!(contenedor_cafe.granos_consumidos, C);
        assert_eq!(contenedor_cafe.cafe_molido, C - 5);
        assert_eq!(contenedor_cafe.granos, G - C);
        assert_eq!(contenedor_espuma.espuma_consumida, 5);
        assert_eq!(contenedor_espuma.leche_consumida, E);
        assert_eq!(contenedor_espuma.espuma, E - 5);
        assert_eq!(contenedor_espuma.leche, L - E);
    }

    #[test]
    fn test04_multiples_pedidos() {
        let cafetera = Cafetera::new();
        let res = cafetera.realizar_pedidos("tests/test04.txt");
        let cant_pedidos = *cafetera.cant_pedidos.lock().unwrap();
        let contenedor_cafe = cafetera.cafe.0.lock().unwrap();
        let contenedor_espuma = cafetera.espuma.0.lock().unwrap();
        assert!(res.is_ok());
        assert_eq!(cant_pedidos, 15);
        assert_eq!(contenedor_cafe.cafe_consumido, 90);
        assert_eq!(contenedor_cafe.granos_consumidos, 140);
        assert_eq!(contenedor_cafe.cafe_molido, 50);
        assert_eq!(contenedor_cafe.granos, 60);
        assert_eq!(contenedor_espuma.espuma_consumida, 66);
        assert_eq!(contenedor_espuma.leche_consumida, 91);
        assert_eq!(contenedor_espuma.espuma, 25);
        assert_eq!(contenedor_espuma.leche, 109);
    }

    #[test]
    fn test05_pedidos_invalidos() {
        let cafetera = Cafetera::new();
        let res = cafetera.realizar_pedidos("tests/test05.txt");
        let cant_pedidos = *cafetera.cant_pedidos.lock().unwrap();
        let contenedor_cafe = cafetera.cafe.0.lock().unwrap();
        let contenedor_espuma = cafetera.espuma.0.lock().unwrap();
        assert!(res.is_ok());
        assert_eq!(cant_pedidos, 2);
        assert_eq!(contenedor_cafe.cafe_consumido, 10);
        assert_eq!(contenedor_cafe.granos_consumidos, 50);
        assert_eq!(contenedor_cafe.cafe_molido, 40);
        assert_eq!(contenedor_cafe.granos, 150);
        assert_eq!(contenedor_espuma.espuma_consumida, 12);
        assert_eq!(contenedor_espuma.leche_consumida, 50);
        assert_eq!(contenedor_espuma.espuma, 38);
        assert_eq!(contenedor_espuma.leche, 150);
    }
}

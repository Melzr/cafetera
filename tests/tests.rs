#[cfg(test)]
mod tests {
    use cafeteria::cafetera::Cafetera;
    use cafeteria::constantes::{C, E, G, L};
    use cafeteria::error::CafeteriaError;

    fn assert_estado_cafetera(
        ruta: &str,
        pedidos: u32,
        cafe: u32,
        granos: u32,
        espuma: u32,
        leche: u32,
        cafe_cons: u32,
        granos_cons: u32,
        espuma_cons: u32,
        leche_cons: u32,
    ) {
        let cafetera = Cafetera::new();
        let res = cafetera.realizar_pedidos(ruta);
        let cant_pedidos = *cafetera.cant_pedidos.lock().unwrap();
        let contenedor_cafe = cafetera.cafe.0.lock().unwrap();
        let contenedor_espuma = cafetera.espuma.0.lock().unwrap();
        assert!(res.is_ok());
        assert_eq!(cant_pedidos, pedidos);
        assert_eq!(contenedor_cafe.cafe_consumido, cafe_cons);
        assert_eq!(contenedor_cafe.granos_consumidos, granos_cons);
        assert_eq!(contenedor_cafe.cafe_molido, cafe);
        assert_eq!(contenedor_cafe.granos, granos);
        assert_eq!(contenedor_espuma.espuma_consumida, espuma_cons);
        assert_eq!(contenedor_espuma.leche_consumida, leche_cons);
        assert_eq!(contenedor_espuma.espuma, espuma);
        assert_eq!(contenedor_espuma.leche, leche);
    }

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
        assert_estado_cafetera(
            "tests/test03.txt",
            1,
            C - 5,
            G - C,
            E - 5,
            L - E,
            5,
            C,
            5,
            E,
        );
    }

    #[test]
    fn test04_multiples_pedidos() {
        assert_estado_cafetera("tests/test04.txt", 15, 50, 60, 25, 109, 90, 140, 66, 91);
    }

    #[test]
    fn test05_pedidos_invalidos() {
        assert_estado_cafetera("tests/test05.txt", 2, 40, 150, 38, 150, 10, 50, 12, 50);
    }
}

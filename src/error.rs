/// Errores posibles del programa.
#[derive(Debug)]
pub enum CafeteriaError {
    /// No se recibió la ruta al archivo con pedidos.
    ArgumentosInvalidos,
    /// No se pudo convertir una línea del archivo de pedidos a un [Pedido](`crate::pedido::Pedido`).
    PedidoInvalido,
    /// No se pudo abrir el archivo de pedidos.
    CreacionArchivo,
    /// No se pudo abrir el archivo de pedidos.
    AperturaArchivo,
    /// No se pudo escribir en el archivo de pedidos.
    EscrituraArchivo,
    /// No se pudo leer una línea del archivo de pedidos.
    LecturaArchivo,
    LockEnvenenado,
}

impl From<std::num::ParseIntError> for CafeteriaError {
    fn from(_: std::num::ParseIntError) -> Self {
        CafeteriaError::PedidoInvalido
    }
}

impl<T> From<std::sync::PoisonError<T>> for CafeteriaError {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        CafeteriaError::LockEnvenenado
    }
}
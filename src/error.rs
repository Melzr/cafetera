#[derive(Debug)]
pub enum CafeteriaError {
    ArgumentosInvalidos,
    PedidoInvalido,
    CreacionArchivo,
    AperturaArchivo,
    EscrituraArchivo,
    LecturaArchivo,
    LockError,
}

impl From<std::num::ParseIntError> for CafeteriaError {
    fn from(_: std::num::ParseIntError) -> Self {
        CafeteriaError::PedidoInvalido
    }
}

impl<T> From<std::sync::PoisonError<T>> for CafeteriaError {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        CafeteriaError::LockError
    }
}
/// Capacidad del contenedor de granos
pub const G: u32 = 1000;
/// Capacidad del contenedor de cafe molido
pub const C: u32 = 100;
/// Capacidad del contenedor de leche
pub const L: u32 = 1000;
/// Capacidad del contenedor de espuma
pub const E: u32 = 100;
/// Cantidad de dispensadores por cafetera
pub const N: usize = 3;
/// Cantidad de cafeteras
pub const CAFETERAS: usize = 3;

/// Tiempo de espera para reponer el contenedor ded cafe molido
pub const TIEMPO_CAFE: u64 = 4000;
/// Tiempo de espera para reponer el contenedor de granos
pub const TIEMPO_GRANOS: u64 = 2000;
/// Tiempo de espera para reponer el contenedor de espuma
pub const TIEMPO_ESPUMA: u64 = 4000;
/// Tiempo de espera para reponer el contenedor de leche
pub const TIEMPO_LECHE: u64 = 2000;
/// Tiempo de espera por unidad de cafe, agua o espuma del pedido
pub const TIEMPO_POR_UNIDAD: u64 = 100;

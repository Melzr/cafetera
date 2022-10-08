/// Capacidad del contenedor de granos
pub const G: u32 = 200;
/// Capacidad del contenedor de cafe molido
pub const C: u32 = 50;
/// Capacidad del contenedor de leche
pub const L: u32 = 200;
/// Capacidad del contenedor de espuma
pub const E: u32 = 50;
/// Cantidad de dispensadores
pub const N: usize = 3;

/// Cantidad mínima de café, espuma y agua
pub const MIN_CANTIDAD: u32 = 1;
/// Cantidad máxima de café, espuma y agua
pub const MAX_CANTIDAD: u32 = 10;

/// Tiempo de espera para reponer el contenedor de cafe molido
pub const TIEMPO_CAFE: u64 = 2000;
/// Tiempo de espera para reponer el contenedor de espuma
pub const TIEMPO_ESPUMA: u64 = 2000;
/// Tiempo de espera por unidad de cafe, agua o espuma del pedido
pub const TIEMPO_POR_UNIDAD: u64 = 100;
/// Tiempo transcurrido hasta mostrar las estadísticas
pub const TIEMPO_STATS: u64 = 5000;

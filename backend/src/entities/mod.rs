// ============================================================================
// ENTITIES MODULE - SeaORM Entity Definitions
// ============================================================================
//
// Purpose: Mendefinisikan entitas database menggunakan SeaORM
//
// Entity showtimes sudah dibuat dan di-export untuk workflow service
// ============================================================================

pub mod showtimes;
pub use showtimes::{Entity as ShowtimesEntity, Model as Showtime};

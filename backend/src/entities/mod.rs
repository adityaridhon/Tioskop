// ============================================================================
// ENTITIES MODULE - SeaORM Entity Definitions
// ============================================================================
//
// Purpose: Mendefinisikan entitas database menggunakan SeaORM
//
// All database tables are mapped to SeaORM entities
// ============================================================================

pub mod booking_seats;
pub mod bookings;
pub mod movies;
pub mod seats;
pub mod showtimes;
pub mod studios;
pub mod users;

// Re-export entities and models for convenience
pub use booking_seats::Entity as BookingSeatsEntity;
pub use bookings::{Entity as BookingsEntity, Model as Booking};
pub use movies::{Entity as MoviesEntity, Model as Movie};
pub use seats::{Entity as SeatsEntity, Model as Seat};
pub use showtimes::{Entity as ShowtimesEntity, Model as Showtime};
pub use studios::{Entity as StudiosEntity, Model as Studio};
pub use users::Entity as UsersEntity;

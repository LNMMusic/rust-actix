// SCHEMAS DB [Driver]
pub mod schema;

// Types
use diesel::{PgConnection, r2d2::{self, ConnectionManager}};
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Handlers
pub mod user;
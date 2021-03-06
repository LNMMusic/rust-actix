// SCHEMA DB [Driver]
pub mod schema;

// DB HANDLERS
use diesel::{PgConnection, r2d2::{self, ConnectionManager}};
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub mod user;
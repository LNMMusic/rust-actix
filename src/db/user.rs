use actix_web::web;
use diesel::RunQueryDsl;

use crate::db::{Pool, schema::users::dsl::users};
use crate::models::user::User;


// READ
pub fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn= pool.get().unwrap();

    let items = users.load::<User>(&conn)?;
    Ok(items)
}

// WRITE
// ...
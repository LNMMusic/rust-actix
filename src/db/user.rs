use actix_web::web;
use diesel::{RunQueryDsl};

use crate::{db::Pool, models::user::User};
use crate::db::schema::users::dsl::*;


// HANDLERS
pub fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn= pool.get().unwrap();

    let items = users.load::<User>(&conn)?;
    Ok(items)
}

pub fn get_user_by_id() -> () {

}
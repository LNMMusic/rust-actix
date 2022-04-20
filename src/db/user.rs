use actix_web::web;
// handlers to db
use diesel::{RunQueryDsl, QueryDsl};    // extra features for table schema
use diesel::dsl::{insert_into, update, delete};

use crate::db::{Pool, schema::users::dsl::users};
use crate::models::user::{UserDB, User};


// READ
pub fn get_user_all(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn= pool.get().unwrap();

    let items = users.load::<User>(&conn)?;
    Ok(items)
}

pub fn get_user_by_id(pool: web::Data<Pool>, id: i32) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let item = users.find(id).get_result::<User>(&conn);
    item
}


// WRITE
pub fn create_user(pool: web::Data<Pool>, user: UserDB) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let item = insert_into(users).values(&user).get_result(&conn)?;
    Ok(item)
}

pub fn update_user(pool: web::Data<Pool>, user: UserDB, id: i32) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let item = update(users.find(id)).set(&user).get_result(&conn)?;
    Ok(item)
}

pub fn delete_user(pool: web::Data<Pool>, id: i32) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let count = delete(users.find(id)).execute(&conn)?;
    Ok(count)
}
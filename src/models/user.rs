use serde::{Serialize, Deserialize};
use crate::db::schema::*;


// DB MODEL
// model to connect to schema [allows diesel to manage transaction to db]
#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct UserDB<'a> {
    pub username:   &'a str,
    pub password:   &'a str,
    pub fullname:   &'a str,
    pub email:      &'a str,
}


// MODEL
// model to use on services - routes
#[derive(Serialize, Deserialize, Default, Queryable)]
pub struct User {
    pub id:         i32,
    pub username:   String,
    pub password:   String,

    pub fullname:   String,
    pub email:      String,
}

pub struct UserRequest {
    pub username:   String,
    pub password:   String,
    pub fullname:   String,
    pub email:      String,
}
impl UserRequest {
    pub fn hash_password(self: &mut Self) {
        self.password = format!("{}_hashed", self.password)
    }
}


use serde::{Serialize, Deserialize};
use crate::db::schema::*;


// DB MODEL
// model to connect to schema [allows diesel to manage transaction to db (write - create)]
#[derive(Debug, Insertable, AsChangeset)]
#[table_name="users"]
pub struct UserDB<'a> {
    // this model is the main that will connect table schema to models
    // from the services - routes [as an input]
    pub username:   &'a str,
    pub password:   &'a str,
    pub fullname:   &'a str,
    pub email:      &'a str,
}
impl<'a> UserDB<'a> {
    // const
    pub fn generate(user: &'a UserRequest) -> Self {
        Self {
            username:   &user.username,
            password:   &user.password,
            fullname:   &user.fullname,
            email:      &user.email,
        }
    }
}


// MODEL
// model to use on services - routes
#[derive(Serialize, Deserialize, Default, Queryable)]
pub struct User {
    // this model is the main that will connect db handlers
    // to services - routes [as an output]
    pub id:         i32,
    pub username:   String,
    pub password:   String,

    pub fullname:   String,
    pub email:      String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserRequest {
    pub username:   String,
    pub password:   String,
    pub fullname:   String,
    pub email:      String,
}
impl UserRequest {
    // methods
    pub fn hash_password(self: &mut Self) {
        self.password = format!("{}_hashed", self.password)
    }
}


use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;


#[derive(Serialize, Deserialize, Default, PostgresMapper)]
#[pg_mapper(table="users")]
pub struct User {
    pub username:   String,
    pub password:   String,

    pub fullname:   String,
    pub email:      String,
}
impl User {
    pub fn hash_password(self: &mut Self) {
        self.password = format!("{}_hashed", self.password)
    }
}
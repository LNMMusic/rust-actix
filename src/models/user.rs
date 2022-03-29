use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Default)]
pub struct User {
    pub id:         i32,
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
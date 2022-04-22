pub mod jwt {
    #[derive(Debug)]
    pub struct JWTConfig {
        pub header: i32,
        pub key:    &'static str,
    }

    pub const JWTCONFIG: JWTConfig = JWTConfig {
        header: 3,
        key:    "secret"
    };
}
use crate::jwt::JWTCONFIG;

pub fn read_header() {
    println!("my header is {} and my key {}", JWTCONFIG.header, JWTCONFIG.key)
}

#[test]
fn testing() {
    read_header()
}
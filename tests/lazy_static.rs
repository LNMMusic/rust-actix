#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
pub struct JWTConfig {
    header: Vec<i32>,
    key:    &'static str,
}

lazy_static! {
    static ref JWTCONFIG: JWTConfig = JWTConfig { header: vec![1,2,3], key: "secret" };
}


pub mod inner {
    use crate::JWTCONFIG;

    pub fn run() {
        println!("{:#?}",*JWTCONFIG)
    }
}


#[test]
fn testing() {
    use crate::inner;
    inner::run()
}
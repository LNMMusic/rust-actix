#[macro_use]
extern crate lazy_static;


pub mod jwt {
    use serde::{Serialize, Deserialize};
    use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
    use std::time::{SystemTime, UNIX_EPOCH, Duration};
    
    pub fn current_time() -> Duration {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
    }

    const JWT_SECRET: &[u8] = b"secret";
    const JWT_EXP_TIME: u64 = 3;            // seconds


    #[derive(Debug, Serialize, Deserialize)]
    pub struct Claims {
        pub id:     i32,
        pub admin:  bool,
        pub iat:    usize,
        pub exp:    usize,
    }
    impl Claims {
        // constr
        pub fn new(id: i32, admin: bool) -> Self {
            let iat = current_time();
            let exp = iat + Duration::from_secs(JWT_EXP_TIME);

            Self { id, admin, iat: iat.as_secs() as usize, exp: exp.as_secs() as usize }
        }
        // methods
        pub fn is_expired(&self) -> bool {
            if current_time().as_secs() as usize >= self.exp {
                return true
            }
            false
        }
    }

    pub struct JWTEncoder {
        key:    EncodingKey,
        header: Header,
    }
    impl JWTEncoder {
        pub fn create_token(&self, claims: Claims) -> Result<String, String> {
            let token = encode(&self.header, &claims, &self.key); if token.is_err() {
                return Err("Failed to create token!".to_owned())
            }
            Ok(token.unwrap())
        }
    }
    pub struct JWTDecoder {
        key:    DecodingKey,
        valid:  Validation,
    }
    impl JWTDecoder {
        pub fn verify_token(&self, token: String) -> Result<Claims, String> {
            let claims = decode::<Claims>(&token, &self.key, &self.valid); if claims.is_err() {
                return Err("Failed to get claims from jwt token!".to_owned())
            };
            let claims = claims.unwrap().claims; if claims.is_expired() {
                return Err("Invalid token. It has expired!".to_owned())
            };
            Ok(claims)
        }
    }

    pub struct JWTConfig {
        pub encoder:    JWTEncoder,
        pub decoder:    JWTDecoder,
    }
    impl JWTConfig {
        // constr
        pub fn new() -> Self {
            Self {
                encoder:    JWTEncoder {
                    key:    EncodingKey::from_secret(JWT_SECRET),
                    header: Header::new(Algorithm::HS256),
                },
                decoder:    JWTDecoder {
                    key:    DecodingKey::from_secret(JWT_SECRET),
                    valid:  Validation::new(Algorithm::HS256),
                },
            }
        }
    }
    
    // Instance
    lazy_static! {
        pub static ref JWTCONFIG: JWTConfig = JWTConfig::new();
    }
}



use crate::jwt::{JWTCONFIG, Claims};
use std::{thread, time};

#[test]
fn testing() {
    // create token
    let claims = Claims::new(3, true);
    let token = JWTCONFIG.encoder.create_token(claims).unwrap();
    println!("token\t->{}", token);

    // verify token
    let data = JWTCONFIG.decoder.verify_token(token).unwrap();
    println!("claims:\n{:#?}", data);
}

#[test]
fn expired() {
    // create token
    let claims = Claims::new(3, true);
    let token = JWTCONFIG.encoder.create_token(claims).unwrap();
    println!("token\t->{}", token);

    // expiration
    thread::sleep(time::Duration::from_secs(5));

    // verify token
    let data = JWTCONFIG.decoder.verify_token(token);
    assert_eq!(data.is_err(), true);
}
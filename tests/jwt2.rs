use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, errors};


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // custom
    id:     i32,
    admin:  bool,
    // default
    exp:    usize,
}


fn create_token(claims: Claims) -> Result<String, errors::Error> {
    let header = &Header::new(Algorithm::HS256);
    let key = &EncodingKey::from_secret(b"secret");

    let token = encode(header, &claims, key);
    token
}
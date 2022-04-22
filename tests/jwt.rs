use jwt_simple::prelude::*;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimsUser {
    id:         i32,
    is_admin:   bool,
}

pub fn create_token(key_sign: &HS256Key, claims_user: ClaimsUser) -> Result<String, String> {
    // create a token by signing the claims (w/ custom data) with the key
    // let key_sign = HS256Key::generate();
    let claims = Claims::with_custom_claims(
        claims_user,
        Duration::from_secs(3)
    );

    let token = key_sign.authenticate(claims); if token.is_err() {
        return Err("Failed to create token during claims signing!".to_owned())
    }
    Ok(token.unwrap())
}

pub fn verify_token(key_sign: &HS256Key, token: String) -> Result<JWTClaims<ClaimsUser>, String> {
    // verify a token with the key sign (returning the claims)
    // let key_sign = HS256Key::generate();
    let mut options = VerificationOptions::default();
    options.time_tolerance = None;

    let claims = key_sign.verify_token::<ClaimsUser>(&token, Some(options)); if claims.is_err() {
        return Err("Failed to verify token, it may be invalid or expired!".to_owned())
    };
    Ok(claims.unwrap())
}


#[test]
fn testing() {
    // key to create and verify tokens
    let key_sign = HS256Key::generate();

    // create
    let token = create_token(&key_sign, ClaimsUser {
        id:         3,
        is_admin:   false,
    }).unwrap();
    println!("token -> {:#?}", token);

    // verify
    let claims = verify_token(&key_sign, token).unwrap();
    println!("claims -> {:#?}", claims);

    assert_eq!(claims.custom.id, 3)
}

#[test]
fn token_expired() {
    // key to create and verify tokens
    let key_sign = HS256Key::generate();

    // create
    let token = create_token(&key_sign, ClaimsUser {
        id:         3,
        is_admin:   false,
    }).unwrap();
    println!("token -> {:#?}", token);
    
    // time to expirate token
    std::thread::sleep(std::time::Duration::from_secs(5));

    // verify
    let claims = verify_token(&key_sign, token);
    
    assert_eq!(claims.is_err(), true)
}
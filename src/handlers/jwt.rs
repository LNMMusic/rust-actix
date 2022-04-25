use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
fn current_time() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}

const JWT_SECRET: &[u8] = b"secret";
const JWT_EXP_TIME: u64 = 900;          // seconds


// CLAIMS
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


// JWT Handler
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
    #[warn(dead_code)]
    valid:  Validation,
}
impl JWTDecoder {
    pub fn _verify_token(&self, token: String) -> Result<Claims, String> {
        let claims = decode::<Claims>(&token, &self.key, &self.valid); if claims.is_err() {
            return Err("Failed to get claims from jwt token!".to_owned())
        };
        let claims = claims.unwrap().claims; if claims.is_expired() {
            return Err("Invalid token. It has expired!".to_owned())
        };
        Ok(claims)
    }
    pub fn valid_token(&self, token: String) -> bool {
        let claims = decode::<Claims>(&token, &self.key, &self.valid); if claims.is_err() {
            return false
        };
        let claims = claims.unwrap().claims; if claims.is_expired() {
            return false
        };
        true
    }
}

pub struct JWTConfig {
    pub encoder:    JWTEncoder,
    pub decoder:    JWTDecoder,
}
impl JWTConfig {
    // constr
    pub fn new(key: &[u8]) -> Self {
        Self {
            encoder:    JWTEncoder {
                key:    EncodingKey::from_secret(key),
                header: Header::new(Algorithm::HS256),
            },
            decoder:    JWTDecoder {
                key:    DecodingKey::from_secret(key),
                valid:  Validation::new(Algorithm::HS256),
            }
        }
    }
}

// Instance
lazy_static! {
    pub static ref JWTCONFIG: JWTConfig = JWTConfig::new(JWT_SECRET);
}


// MIDDLEWARE
pub mod middleware {
    use std::future::{ready, Ready};
    use actix_web::{
        dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
        Error,
    };
    use futures_util::future::LocalBoxFuture;


    pub struct JWT;
    
    impl<S, B> Transform<S, ServiceRequest> for JWT
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
    {
        type Response = ServiceResponse<B>;
        type Error = Error;
        type InitError = ();
        type Transform = JWTMiddleware<S>;
        type Future = Ready<Result<Self::Transform, Self::InitError>>;

        fn new_transform(&self, service: S) -> Self::Future {
            ready(Ok(JWTMiddleware { service }))
        }
    }

    // Main Struct
    pub struct JWTMiddleware<S> {
        service:    S,
    }
    impl<S, B> Service<ServiceRequest> for JWTMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
    {
        type Response = ServiceResponse<B>;
        type Error = Error;
        type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    
        forward_ready!(service);
    
        fn call(&self, req: ServiceRequest) -> Self::Future {
            println!("Hi from start. You requested: {}", req.path());
    
            let fut = self.service.call(req);
    
            Box::pin(async move {
                let res = fut.await?;
    
                println!("Hi from response");
                Ok(res)
            })
        }
    }
}

pub mod middleware2 {
    use actix_web_httpauth::extractors::{
        bearer::{BearerAuth, Config},
        AuthenticationError
    };
    use actix_web::{
        dev::{ServiceRequest},
        Error,
    };
    use super::JWTCONFIG;


    pub async fn jwt_validation(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
        let config = req
            .app_data::<Config>()
            .map(|data| data.clone())
            .unwrap_or_else(Default::default);

        if JWTCONFIG.decoder.valid_token(credentials.token().to_owned()) {
            return Ok(req)
        };
        Err(AuthenticationError::from(config).into())
    }
}
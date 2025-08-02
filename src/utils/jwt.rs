use std::future;

use actix_web::{FromRequest, HttpMessage};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use super::constants;


#[derive(Serialize,Deserialize,Clone)]
pub struct Claims{
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub id: i32
}

impl FromRequest for Claims{
    type Error = actix_web::Error;

    type Future = future::Ready<Result<Self,Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest, 
        _payload: &mut actix_web::dev::Payload
    ) -> std::future::Ready<Result<Claims,actix_web::Error>> {
        match req.extensions().get::<Claims>() {
            Some(claim) => future::ready(Ok(claim.clone())),
            None => future::ready(Err(actix_web::error::ErrorBadRequest("Bad Claims")))
        }
    }
}

pub fn encode_jwt(email: String, id: i32) -> Result<String,jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expire = Duration::hours(24);

    let claims = Claims{
        exp: (now+expire).timestamp() as usize,
        iat: now.timestamp() as  usize,
        email,
        id,
    };

    let secret = (*constants::SECRET).clone();

    encode(&Header::default(), &claims,&EncodingKey::from_secret(secret.as_ref()))
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>,jsonwebtoken::errors::Error> {
    let secret = (*constants::SECRET).clone();
    let claim_data: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = decode(
        &jwt, 
        &DecodingKey::from_secret(secret.as_ref()), 
        &Validation::default()
    );

    claim_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_jwt_success() {
        let email = "test@example.com".to_string();
        let id = 123;
        
        let result = encode_jwt(email.clone(), id);
        
        assert!(result.is_ok());
        let token = result.unwrap();
        assert!(!token.is_empty());
    }

    #[test]
    fn test_decode_jwt_success() {
        let email = "test@example.com".to_string();
        let id = 123;
        
        // First encode a JWT
        let token = encode_jwt(email.clone(), id).unwrap();
        
        // Then decode it
        let result = decode_jwt(token);
        
        assert!(result.is_ok());
        let token_data = result.unwrap();
        assert_eq!(token_data.claims.email, email);
        assert_eq!(token_data.claims.id, id);
    }

    #[test]
    fn test_decode_invalid_jwt() {
        let invalid_token = "invalid.jwt.token".to_string();
        
        let result = decode_jwt(invalid_token);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_claims_struct() {
        let now = Utc::now();
        let claims = Claims {
            exp: (now + chrono::Duration::hours(1)).timestamp() as usize,
            iat: now.timestamp() as usize,
            email: "test@example.com".to_string(),
            id: 456,
        };

        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.id, 456);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_jwt_round_trip_with_different_users() {
        let test_cases = vec![
            ("user1@test.com", 1),
            ("user2@example.org", 999),
            ("admin@company.com", 42),
        ];

        for (email, id) in test_cases {
            let token = encode_jwt(email.to_string(), id).unwrap();
            let decoded = decode_jwt(token).unwrap();
            
            assert_eq!(decoded.claims.email, email);
            assert_eq!(decoded.claims.id, id);
        }
    }
}
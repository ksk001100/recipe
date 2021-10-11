use crate::errors::ServiceError;
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    http::{self, Response, StatusCode, Request},
    AddExtensionLayer
};
use headers::{authorization::Bearer, Authorization};
use tower_http::auth::{RequireAuthorizationLayer, AuthorizeRequest};
use axum::body::Body;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[async_trait]
impl<B> FromRequest<B> for Claims
    where
        B: Send + Debug
{
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await.unwrap();

        match validate_token(bearer.token()).await {
            // Ok(claims) => if res == true {
            //     println!("{:?}", req);
            //     Ok(Claims { sub: String::from("Hello"), company: String::from("Hello"), exp: 121212 })
            // } else {
            //     Err(StatusCode::UNAUTHORIZED)
            // },
            // Err(_) => Err(StatusCode::UNAUTHORIZED)
        // }
            Ok(claims) => Ok(claims),
            Err(_) => Err(StatusCode::UNAUTHORIZED)
        }
        // let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
        //     .map_err(|_| AuthError::InvalidToken)?;

        // Ok(token_data.claims)
    }
}

#[derive(Debug)]
enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

pub async fn validate_token(token: &str) -> Result<Claims, ServiceError> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let jwks = fetch_jwks(&format!("{}{}", authority.as_str(), ".well-known/jwks.json")).await
        .expect("failed to fetch jwks");
    let validations = vec![Validation::Issuer(authority), Validation::SubjectPresent];
    let kid = match token_kid(&token) {
        Ok(res) => res.expect("failed to decode kid"),
        Err(_) => return Err(ServiceError::JWKSFetchError(String::from("Could not fetch JWKS"))),
    };
    let jwk = jwks.find(&kid).expect("Specified key not found in set");
    let res = validate(token, jwk, validations);
    let claims = res.ok().unwrap().claims;
    println!("Claims: {:?}", claims);
    Ok(
        Claims {
            sub: claims.get("sub").unwrap().to_string(),
            exp: claims.get("exp").unwrap().to_string().parse().unwrap(),
            company: "Hello.world".to_string() // TODO Auth0 custom claims
        }
    )
    // Ok(res.is_ok())
    // Ok(true)
}

async fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let mut res = reqwest::get(uri).await.unwrap();
    let val = res.json::<JWKS>().await.unwrap();
    return Ok(val);
}
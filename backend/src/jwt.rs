use axum::{
    Json, RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::IntoResponse,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::config::jwt::get_jwt_opts;

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    pub user_id: u32,
    exp: usize,
}

impl JwtClaims {
    pub fn new(user_id: u32) -> Self {
        JwtClaims {
            user_id,
            exp: usize::MAX,
        }
    }

    pub fn gen_token(&self) -> String {
        let jwt_opts = get_jwt_opts();

        let secret = jwt_opts.secret.as_bytes();

        encode(&Header::default(), &self, &EncodingKey::from_secret(secret)).unwrap()
    }

    pub fn parse_token(token: String) -> Result<JwtClaims, impl IntoResponse> {
        let jwt_opts = get_jwt_opts();

        let secret = jwt_opts.secret.as_bytes();

        match decode::<Self>(
            &token,
            &DecodingKey::from_secret(secret),
            &Validation::default(),
        ) {
            Ok(claim) => Ok(claim.claims),
            Err(_) => Err(Errors::InvalidToken),
        }
    }
}

pub enum Errors {
    InvalidToken,
}

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "Invalid token".to_string()
            })),
        )
            .into_response()
    }
}

impl<S> FromRequestParts<S> for JwtClaims
where
    S: Send + Sync,
{
    type Rejection = Errors;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Ok(TypedHeader(Authorization(bearer))) =
            parts.extract::<TypedHeader<Authorization<Bearer>>>().await
        else {
            return Err(Errors::InvalidToken);
        };

        let user_data =
            JwtClaims::parse_token(bearer.token().to_string()).map_err(|_| Errors::InvalidToken);

        user_data
    }
}

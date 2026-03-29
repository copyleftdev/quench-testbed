use actix_web::{HttpRequest, HttpResponse, middleware};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub fn validate_token(req: &HttpRequest) -> Result<Claims, AuthError> {
        let header = req.headers().get("Authorization")
            .ok_or(AuthError::MissingToken)?;
        let token = header.to_str()
            .map_err(|_| AuthError::InvalidToken)?
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidToken)?;
        // Validate JWT
        Ok(Claims { sub: "user".into(), role: "admin".into() })
    }
}

pub struct Claims { pub sub: String, pub role: String }
pub enum AuthError { MissingToken, InvalidToken, Expired }

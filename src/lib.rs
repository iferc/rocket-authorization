pub mod basic;
pub mod oauth;

pub use rocket::Request;

use core::ops::DerefMut;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use std::fmt::Debug;
use std::ops::Deref;

#[rocket::async_trait]
pub trait Authorization: Sized {
    const KIND: &'static str;
    async fn parse(kind: &str, credential: &str, request: &Request) -> Result<Self, AuthError>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Credential<AuthorizationType>(pub AuthorizationType);

impl<T> Deref for Credential<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Credential<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<AuthorizationType> Credential<AuthorizationType> {
    pub fn into_inner(self) -> AuthorizationType {
        self.0
    }
}

/// Note that IncompatibleKind and HeaderMissing will trigger a Bad Request response
/// if used in a trait implementation as they are meant for internal use.
#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum AuthError {
    #[error("Authorization header is missing.")]
    HeaderMissing,

    #[error("Authorization header is malformed.")]
    HeaderMalformed,

    #[error("Authorization kind is incompatible.")]
    IncompatibleKind,

    #[error("Authorization details could not be parsed.")]
    Unprocessable(String),

    #[error("Access is unauthorized.")]
    Unauthorized,

    #[error("Provided credentials are forbidden.")]
    Forbidden,

    #[error("Payment is required for access.")]
    PaymentRequired,

    #[error("{0}")]
    Status(Status),
}

#[rocket::async_trait]
impl<'r, AuthorizationType: Authorization> FromRequest<'r> for Credential<AuthorizationType> {
    type Error = AuthError;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<Self, (Status, <Self as FromRequest<'r>>::Error), Status> {
        match request.headers().get_one("Authorization") {
            None => Outcome::Error((Status::Unauthorized, AuthError::HeaderMissing)),
            Some(authorization_header) => {
                let header_sections: Vec<_> = authorization_header.split_whitespace().collect();

                if header_sections.len() != 2 {
                    return Outcome::Error((Status::BadRequest, AuthError::HeaderMalformed));
                }

                let (kind, credential) = (header_sections[0], header_sections[1]);

                if AuthorizationType::KIND != kind {
                    return Outcome::Error((Status::Unauthorized, AuthError::IncompatibleKind));
                }

                match AuthorizationType::parse(kind, credential, request).await {
                    Ok(credentials) => Outcome::Success(Credential(credentials)),

                    Err(error @ AuthError::HeaderMissing)
                    | Err(error @ AuthError::Unauthorized) => {
                        Outcome::Error((Status::Unauthorized, error))
                    }

                    Err(error @ AuthError::IncompatibleKind)
                    | Err(error @ AuthError::Forbidden) => {
                        Outcome::Error((Status::Forbidden, error))
                    }

                    Err(error @ AuthError::PaymentRequired) => {
                        Outcome::Error((Status::PaymentRequired, error))
                    }

                    Err(error @ AuthError::HeaderMalformed) => {
                        Outcome::Error((Status::BadRequest, error))
                    }

                    Err(error @ AuthError::Unprocessable(_)) => {
                        Outcome::Error((Status::UnprocessableEntity, error))
                    }

                    Err(AuthError::Status(status)) => {
                        Outcome::Error((status, AuthError::Status(status)))
                    }
                }
            }
        }
    }
}

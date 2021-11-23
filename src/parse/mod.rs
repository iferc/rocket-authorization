pub use rocket::Request;

use rocket::{http::Status, outcome::Outcome, request::FromRequest};
use std::fmt::Debug;

pub mod basic;
pub mod oauth;

/// Note that IncompatibleKind and HeaderMissing will trigger a Bad Request response
/// if used in a trait implementation as they are meant for internal use.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Responds with HTTP Unauthorized
    Unauthorized,
    /// Responds with HTTP Unauthorized
    IncompatibleKind,
    /// Responds with HTTP Unauthorized
    HeaderMissing,
    /// Responds with HTTP Bad Request
    HeaderMalformed,
    /// Responds with HTTP Bad Request
    CredentialMalformed(String),
}

pub trait Authorization: Sized {
    const KIND: &'static str;
    fn parse(kind: &str, credential: &str, request: &Request) -> Result<Self, ParseError>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Credential<AuthorizationType>(AuthorizationType);

use std::ops::Deref;
impl<T> Deref for Credential<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r, AuthorizationType: Authorization> FromRequest<'r> for Credential<AuthorizationType> {
    type Error = ParseError;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<Self, (Status, <Self as FromRequest<'r>>::Error), ()> {
        match request.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::Unauthorized, ParseError::HeaderMissing)),
            Some(authorization_header) => {
                let header_sections: Vec<_> = authorization_header.split_whitespace().collect();
                if header_sections.len() != 2 {
                    return Outcome::Failure((Status::BadRequest, ParseError::HeaderMalformed));
                }

                let (kind, credential) = (header_sections[0], header_sections[1]);

                if AuthorizationType::KIND != kind {
                    return Outcome::Failure((Status::Unauthorized, ParseError::IncompatibleKind));
                }

                match AuthorizationType::parse(kind, credential, request) {
                    Err(ParseError::Unauthorized) => {
                        Outcome::Failure((Status::Unauthorized, ParseError::Unauthorized))
                    }
                    Err(err) => Outcome::Failure((Status::BadRequest, err)),
                    Ok(credentials) => Outcome::Success(Credential(credentials)),
                }
            }
        }
    }
}

use rocket::{http::Status, request::FromRequest, Outcome, Request};
use std::fmt::Debug;

pub mod basic;
pub mod oauth;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    NotExists,
    EmptyError,
    IncompatibleKind,
    UnknownKind,
    InvalidHeader,
    UTFParseError,
    Base64DecodeError,
    NonColonPairError,
}

pub trait Authorization: Sized {
    const KIND: &'static str;
    fn parse(kind: &str, credential: &str) -> Result<Self, ParseError>;
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

impl<'a, 'r, AuthorizationType: Authorization> FromRequest<'a, 'r>
    for Credential<AuthorizationType>
{
    type Error = ParseError;

    fn from_request(
        request: &Request,
    ) -> Outcome<Self, (Status, <Self as FromRequest<'a, 'r>>::Error), ()> {
        match request.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::Unauthorized, ParseError::NotExists)),
            Some(authorization_header) => {
                let header_sections: Vec<_> = authorization_header.split_whitespace().collect();
                if header_sections.len() != 2 {
                    return Outcome::Failure((Status::Unauthorized, ParseError::InvalidHeader));
                }

                let (kind, credential) = (header_sections[0], header_sections[1]);

                if AuthorizationType::KIND != kind {
                    return Outcome::Failure((Status::Unauthorized, ParseError::IncompatibleKind));
                }

                match AuthorizationType::parse(kind, credential) {
                    Ok(credentials) => Outcome::Success(Credential(credentials)),
                    Err(err) => Outcome::Failure((Status::BadRequest, err)),
                }
            }
        }
    }
}

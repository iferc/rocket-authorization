extern crate rocket;

mod parse;
mod response;

pub use parse::{AuthorizationCredential, ParseError};
pub use response::{AuthorizationKind, RequestAuthorization};

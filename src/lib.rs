extern crate rocket;

pub mod parse;
pub mod respond;

pub mod prelude {
    use super::*;

    pub use parse::{basic::Basic, oauth::OAuth, Authorization, Credential, ParseError};
    pub use respond::RequestAuthorization;
}

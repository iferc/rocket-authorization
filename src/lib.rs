extern crate rocket;

pub mod parse;
pub mod respond;

pub mod prelude {
    use super::*;

    pub use parse::basic::Basic;
    pub use parse::oauth::OAuth;
    pub use parse::{Authorization, Credential, ParseError};
    pub use respond::request_authorization;
}

use super::{Basic, Credential};
use rocket::{http::Status, request::FromRequest, request::Outcome, Request};

#[derive(Debug)]
pub struct SysAdmin(String);

impl<'a, 'r> FromRequest<'a, 'r> for SysAdmin {
    type Error = ();
    fn from_request(request: &Request) -> Outcome<Self, Self::Error> {
        let provided_auth = match Credential::<Basic>::from_request(request) {
            Outcome::Success(auth) => auth,
            Outcome::Failure(error) => return Outcome::Failure((error.0, ())),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };

        // THIS IS FOR DEMONSTRATION PURPOSES ONLY, THIS IS NOT SECURE USAGE
        // this would be the place where a database lookup might be performed
        let username = std::env::var("SYSADMIN_USERNAME").unwrap_or("root".into());
        let password = std::env::var("SYSADMIN_PASSWORD").unwrap_or("p2ssw0rd".into());
        // these defaults will validate against `Authorization: Basic cm9vdDpwMnNzdzByZA==`

        if provided_auth.username == username && provided_auth.password == password {
            Outcome::Success(SysAdmin(provided_auth.username.clone()))
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

use super::{Basic, Credential};
use rocket::{http::Status, request::FromRequest, request::Outcome, Request};

#[derive(Debug)]
pub struct SysAdmin(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SysAdmin {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let provided_auth = match Credential::<Basic>::from_request(request).await {
            Outcome::Success(auth) => auth,
            Outcome::Error(error) => return Outcome::Error((error.0, ())),
            Outcome::Forward(status) => return Outcome::Forward(status),
        };

        // THIS IS FOR DEMONSTRATION PURPOSES ONLY, THIS IS NOT SECURE USAGE
        // this would be the place where a database lookup might be performed
        let username = std::env::var("SYSADMIN_USERNAME").unwrap_or("root".into());
        let password = std::env::var("SYSADMIN_PASSWORD").unwrap_or("p2ssw0rd".into());
        // these defaults will validate against `Authorization: Basic cm9vdDpwMnNzdzByZA==`

        if provided_auth.username == username && provided_auth.password == password {
            Outcome::Success(SysAdmin(username))
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}

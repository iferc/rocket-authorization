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

        // THIS IS FOR DEMONSTRATION PURPOSES ONLY, THIS IS NOT SECURE USAGE!
        // This would be the place where a database lookup might be performed.
        if provided_auth.username == "root" && provided_auth.password == "p2ssw0rd" {
            Outcome::Success(SysAdmin(provided_auth.into_inner().username))
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}

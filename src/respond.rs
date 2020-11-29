use crate::parse::Authorization;
use rocket::{
    response::{self, Responder, Response},
    Request,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RequestAuthorization<AuthorizationType: Authorization> {
    authorization: AuthorizationType,
    realm: &'static str,
}

impl<'r, AuthorizationType: Authorization> Responder<'r>
    for RequestAuthorization<AuthorizationType>
{
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .raw_status(401, "Unauthorized")
            .raw_header(
                "WWW-Authenticate",
                format!(
                    "{} realm=\"{}\", charset=\"UTF-8\"",
                    AuthorizationType::KIND,
                    self.realm
                ),
            )
            .ok()
    }
}

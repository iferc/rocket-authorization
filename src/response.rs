use rocket::{
    response::{self, Responder, Response},
    Request,
};

#[derive(Debug, Clone, PartialEq)]
pub enum AuthorizationKind {
    Basic,
    Bearer,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RequestAuthorization<'a> {
    kind: AuthorizationKind,
    realm: &'a str,
}

impl RequestAuthorization<'_> {
    pub fn basic(realm: &str) -> RequestAuthorization {
        RequestAuthorization {
            kind: AuthorizationKind::Basic,
            realm,
        }
    }
    pub fn bearer(realm: &str) -> RequestAuthorization {
        RequestAuthorization {
            kind: AuthorizationKind::Bearer,
            realm,
        }
    }
}

impl<'r> Responder<'r> for RequestAuthorization<'_> {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .raw_status(401, "Unauthorized")
            .raw_header(
                "WWW-Authenticate",
                format!(
                    "{} realm=\"{}\", charset=\"UTF-8\"",
                    match self.kind {
                        AuthorizationKind::Basic => "Basic",
                        AuthorizationKind::Bearer => "Bearer",
                    },
                    self.realm
                ),
            )
            .ok()
    }
}

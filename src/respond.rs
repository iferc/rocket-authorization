use crate::parse::Authorization;
use rocket::response::Response;

pub fn request_authorization<'a, AuthorizationType: Authorization>(realm: &'a str) -> Response<'a> {
    Response::build()
        .raw_status(401, "Unauthorized")
        .raw_header(
            "WWW-Authenticate",
            format!(
                "{} realm=\"{}\", charset=\"UTF-8\"",
                AuthorizationType::KIND,
                realm
            ),
        )
        .finalize()
}

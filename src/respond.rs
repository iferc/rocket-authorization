use crate::parse::Authorization;
use rocket::{http::Status, response::Response};

pub fn request_authorization<'a, AuthorizationType: Authorization>(realm: &'a str) -> Response<'a> {
    Response::build()
        .status(Status::Unauthorized)
        .raw_header(
            "WWW-Authenticate",
            format!(
                r#"{} realm="{realm}", charset="UTF-8""#,
                AuthorizationType::KIND
            ),
        )
        .finalize()
}

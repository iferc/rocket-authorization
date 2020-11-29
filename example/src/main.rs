#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::{Request, Response};
use rocket_authorization::prelude::*;

mod custom_auth;
mod sysadmin_guard;
use custom_auth::CustomAuth;
use sysadmin_guard::SysAdmin;

#[get("/")]
fn index() -> &'static str {
    "ok"
}

#[get("/auth/basic")]
fn auth_basic(auth: Credential<Basic>) -> String {
    format!("success with {}", auth.username)
}

#[get("/auth/basic_safe")]
fn auth_basic_safe(auth: Result<Credential<Basic>, ParseError>) -> String {
    match auth {
        Ok(credential) => format!("success with {}", credential.username),
        Err(_) => "failed".into(),
    }
}

#[get("/auth/bearer")]
fn auth_bearer(auth: Credential<OAuth>) -> String {
    format!("success with {}", auth.token)
}

#[get("/auth/bearer_safe")]
fn auth_bearer_safe(auth: Result<Credential<OAuth>, ParseError>) -> String {
    match auth {
        Ok(credential) => format!("success with {}", credential.token),
        Err(_) => "failed".into(),
    }
}

#[get("/auth/custom")]
fn auth_custom(auth: Credential<CustomAuth>) -> String {
    format!("success with {} for {}", auth.token, auth.slug)
}

#[get("/auth/custom_safe")]
fn auth_custom_safe(auth: Result<Credential<CustomAuth>, ParseError>) -> String {
    match auth {
        Ok(credential) => format!("success with {} for {}", credential.token, credential.slug),
        Err(_) => "failed".into(),
    }
}

#[get("/secure/sysadmin")]
fn secure_sysadmin(user: SysAdmin) -> String {
    format!("success with {}", user.0)
}

#[catch(401)]
fn not_authorized<'a>(_: &Request) -> Response<'a> {
    request_authorization::<Basic>("Example Rocket Web Server")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                auth_basic,
                auth_basic_safe,
                auth_bearer,
                auth_bearer_safe,
                auth_custom,
                auth_custom_safe,
                secure_sysadmin,
            ],
        )
        .register(catchers![not_authorized])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn root_available() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("ok".into()));
    }
}

use rocket::{get, routes, Build};
use rocket_authorization::basic::Basic;
use rocket_authorization::oauth::OAuth;
use rocket_authorization::{AuthError, Credential};

mod custom_auth;
mod sysadmin_guard;
use custom_auth::CustomAuth;
use sysadmin_guard::SysAdmin;

#[get("/")]
fn index() -> &'static str {
    "ok"
}

#[get("/auth/basic_only")]
fn auth_basic_only(auth: Credential<Basic>) -> String {
    // This function only executes with valid basic authentication credentials.

    // Note that the `Basic` type extracts a username and password,
    // but you still need to do your own password validation.

    format!("Hello {}!", auth.username)
}

#[get("/auth/basic_maybe")]
fn auth_basic_maybe(auth: Result<Credential<Basic>, AuthError>) -> String {
    match auth {
        Ok(credential) => format!("Hello {}!", credential.username),

        Err(error) => {
            // Since we extract a `Result<Credential<_>, AuthError>`,
            // we can have custom handling of not being authenticated.

            format!("Error {error}!")
        }
    }
}

#[get("/auth/bearer_only")]
fn auth_bearer_only(auth: Credential<OAuth>) -> String {
    // This function only executes with valid OAuth bearer authentication token.

    // Note that the `OAuth` type extracts a bearer token,
    // but you still need to do your own password validation.

    format!("Token {}!", auth.token)
}

#[get("/auth/bearer_maybe")]
fn auth_bearer_maybe(auth: Result<Credential<OAuth>, AuthError>) -> String {
    match auth {
        Ok(credential) => format!("Token {}!", credential.token),

        Err(error) => {
            // Since we extract a `Result<Credential<_>, AuthError>`,
            // we can have custom handling of not being authenticated.

            format!("Error {error}!")
        }
    }
}

#[get("/auth/custom_only")]
fn auth_custom_only(auth: Credential<CustomAuth>) -> String {
    // This function only executes with valid authentication value.

    format!("Got token {} for {}!", auth.token, auth.slug)
}

#[get("/auth/custom_maybe")]
fn auth_custom_maybe(auth: Result<Credential<CustomAuth>, AuthError>) -> String {
    match auth {
        Ok(credential) => format!("Got token {} for {}!", credential.token, credential.slug),

        Err(error) => {
            // Since we extract a `Result<Credential<_>, AuthError>`,
            // we can have custom handling of not being authenticated.

            format!("Error {error}!")
        }
    }
}

#[get("/secure/sysadmin")]
fn secure_sysadmin(user: SysAdmin) -> String {
    // This function only executes with valid authentication value.

    format!("Hello {}!", user.0)
}

fn rocket() -> rocket::Rocket<Build> {
    rocket::build().mount(
        "/",
        routes![
            index,
            auth_basic_only,
            auth_basic_maybe,
            auth_bearer_only,
            auth_bearer_maybe,
            auth_custom_only,
            auth_custom_maybe,
            secure_sysadmin,
        ],
    )
}

#[rocket::main]
async fn main() {
    let _ = rocket().launch().await.expect("Failed to launch server.");
}

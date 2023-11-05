# Authorization Parsing for Rocket.rs

A library for [Rocket](https://github.com/SergioBenitez/Rocket) web servers to easily access and parse `Authorization` headers from requests in the form of request guards. There is no functionality for performing authentication or generating valid login tokens.

The most common use case is for web micro-services where authentication has already happened elswhere, and the micro-service only needs to use and validate already generated authorization credentials.

Please look at the source within the [`example`](./example) directory for sample usage until proper documentation has been added.

[API documentation](https://docs.rs/rocket-authorization/1.0.0/rocket_authorization/) is available on [docs.rs](https://docs.rs/).

## Installation

To use this crate, run `cargo add rocket-authorization`, or manually add the following to your `Cargo.toml` file:

```toml
[dependencies]
rocket-authorization = "1.0.0"
```

## Usage

### Extract username and password of a `basic` authentication header

```rust
use rocket::get;
use rocket_authorization::basic::Basic;
use rocket_authorization::{AuthError, Credential};

#[get("/auth/basic_only")]
async fn auth_basic_only(auth: Credential<Basic>) -> String {
    // This function only executes with valid basic authentication credentials.

    // Note that the `Basic` type extracts a username and password,
    // but you still need to do your own password validation.

    let user = user.fetch(auth.username, auth.password).await?;

    format!("Hello {}!", user.name)
}

#[get("/auth/basic_maybe")]
async fn auth_basic_maybe(auth: Result<Credential<Basic>, AuthError>) -> String {
    match auth {
        Ok(credential) => {
            let user = user.fetch(auth.username, auth.password).await?;

            format!("Hello {}!", user.name)
        },

        Err(error) => {
            // Since we extract a `Result<Credential<_>, AuthError>`,
            // we can have custom handling of not being authenticated.

            format!("Error {error}!")
        }
    }
}
```

```sh
# Executes route handler.
curl localhost:8000/auth/basic_only -u username:password

# Executes `Ok` case of route handler.
curl localhost:8000/auth/basic_maybe -u username:password

# Executes `Err` case of route handler.
curl localhost:8000/auth/basic_maybe
```

### Extract a bearer token of an OAuth authentication header

```rust
use rocket::get;
use rocket_authorization::oauth::OAuth;
use rocket_authorization::{AuthError, Credential};

#[get("/auth/bearer_only")]
async fn auth_bearer_only(auth: Credential<OAuth>) -> String {
    // This function only executes with valid OAuth bearer authentication token.

    // Note that the `OAuth` type extracts a bearer token,
    // but you still need to do your own password validation.

    let user = user.fetch(auth.token).await?;

    format!("Hello {}!", user.name)
}

#[get("/auth/bearer_maybe")]
async fn auth_bearer_maybe(auth: Result<Credential<OAuth>, AuthError>) -> String {
    match auth {
        Ok(credential) => {
            let user = user.fetch(credential.token).await?;

            format!("Hello {}!", user.name)
        },

        Err(error) => {
            // Since we extract a `Result<Credential<_>, AuthError>`,
            // we can have custom handling of not being authenticated.

            format!("Error {error}!")
        }
    }
}
```

```sh


# Executes route handler.
curl localhost:8000/auth/bearer_only -H 'Authorization: Bearer SomeTokenHere'

# Executes `Ok` case of route handler.
curl localhost:8000/auth/bearer_maybe -H 'Authorization: Bearer SomeTokenHere'

# Executes `Err` case of route handler.
curl localhost:8000/auth/bearer_maybe
```

### Extract a custom authentication header

```rust
use rocket::get;
use rocket_authorization::{AuthError, Authorization, Request};

#[derive(Debug)]
pub struct CustomAuth {
    pub slug: String,
    pub token: String,
}

#[rocket::async_trait]
impl Authorization for CustomAuth {
    const KIND: &'static str = "Custom";

    async fn parse(_: &str, credential: &str, request: &Request) -> Result<Self, AuthError> {
        let components: Vec<_> = credential.split(":").collect();
        if components.len() != 2 {
            return Err(AuthError::Unprocessable(
                "Invalid Key-Value Pair Format Error".into(),
            ));
        }

        let (slug, token) = (components[0].trim(), components[1].trim());
        if slug.is_empty() || token.is_empty() {
            return Err(AuthError::HeaderMissing);
        }

        Ok(CustomAuth {
            slug: slug.into(),
            token: token.into(),
        })
    }
}

#[get("/auth/custom_only")]
async fn auth_custom_only(auth: Credential<CustomAuth>) -> String {
    // This function only executes with valid authentication value.

    let user = user.fetch(auth.token, auth.slug).await?;

    format!("Hello {}!", user.name)
}

#[get("/auth/custom_maybe")]
async fn auth_custom_maybe(auth: Result<Credential<CustomAuth>, AuthError>) -> String {
    match auth {
        Ok(credential) => {
            let user = user.fetch(auth.token, auth.slug).await?;

            format!("Hello {}!", user.name)
        },

        Err(error) => {
            // Since we extract a `Result<Credential<_>, AuthError>`,
            // we can have custom handling of not being authenticated.

            format!("Error {error}!")
        }
    }
}
```

```sh
# Executes route handler.
curl localhost:8000/auth/custom_only -H 'Authorization: Custom SomeTokenHere:SomeSlugHere'

# Executes `Ok` case of route handler.
curl localhost:8000/auth/custom_maybe -H 'Authorization: Custom SomeTokenHere:SomeSlugHere'

# Executes `Err` case of route handler.
curl localhost:8000/auth/custom_maybe
```

### Extracting an authentication header from other request extractors

Credentials can be parsed from anywhere you have access to a `Request` value,
making it easy to compose bespoke authentication guards for routes.

```rust
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

#[get("/secure/sysadmin")]
fn secure_sysadmin(user: SysAdmin) -> String {
    // This function only executes with valid authentication value.

    format!("Hello {}!", user.0)
}
```

```sh
# Executes route handler.
curl localhost:8000/secure/sysadmin -u root:p2ssw0rd

# Responds with 401 Unauthorized and never executes route handler.
curl localhost:8000/secure/sysadmin -u some:other

# Responds with 401 Unauthorized and never executes route handler.
curl localhost:8000/auth/custom_maybe
```

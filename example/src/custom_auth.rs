use rocket_authorization::{AuthError, Authorization, Request};

#[derive(Debug)]
pub struct CustomAuth {
    pub slug: String,
    pub token: String,
}

#[rocket::async_trait]
impl Authorization for CustomAuth {
    const KIND: &'static str = "Custom";

    async fn parse(_: &str, credential: &str, _request: &Request) -> Result<Self, AuthError> {
        let components: Vec<_> = credential.split(":").collect();
        if components.len() != 2 {
            return Err(AuthError::Unprocessable(
                "Invalid Key-Value Pair Format Error".into(),
            ));
        }

        let (slug, token) = (components[0].trim(), components[1].trim());
        if slug.len() == 0 || token.len() == 0 {
            return Err(AuthError::HeaderMissing);
        }

        Ok(CustomAuth {
            slug: slug.into(),
            token: token.into(),
        })
    }
}

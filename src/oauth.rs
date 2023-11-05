use super::{AuthError, Authorization, Request};

#[derive(Debug, Clone, PartialEq)]
pub struct OAuth {
    pub token: String,
}

#[rocket::async_trait]
impl Authorization for OAuth {
    const KIND: &'static str = "Bearer";

    async fn parse(_: &str, credential: &str, _: &Request) -> Result<Self, AuthError> {
        let decoded_text = String::from_utf8(credential.into())
            .map_err(|error| AuthError::Unprocessable(format!("UTF8 Parse Error: {error}")))?;

        let token_str = decoded_text.trim();
        if token_str.is_empty() {
            return Err(AuthError::HeaderMissing);
        }

        Ok(OAuth {
            token: token_str.into(),
        })
    }
}

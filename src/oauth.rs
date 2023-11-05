use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OAuth {
    pub token: String,
}

#[rocket::async_trait]
impl Authorization for OAuth {
    const KIND: &'static str = "Bearer";

    async fn parse(_: &str, credential: &str, _: &Request) -> Result<Self, Error> {
        let decoded_text = String::from_utf8(credential.into())
            .map_err(|error| Error::Unprocessable(format!("UTF8 Parse Error: {error}")))?;

        let token_str = decoded_text.trim();
        if token_str.is_empty() {
            return Err(Error::HeaderMissing);
        }

        Ok(OAuth {
            token: token_str.into(),
        })
    }
}

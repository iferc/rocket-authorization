use super::*;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine as _;

#[derive(Debug, Clone, PartialEq)]
pub struct Basic {
    pub username: String,
    pub password: String,
}

#[rocket::async_trait]
impl Authorization for Basic {
    const KIND: &'static str = "Basic";

    async fn parse(_: &str, credential: &str, _: &Request) -> Result<Self, Error> {
        let decoded_payload = BASE64
            .decode(credential)
            .map_err(|error| Error::Unprocessable(format!("Base64 Decode Error: {error}")))?;

        let decoded_text = String::from_utf8(decoded_payload)
            .map_err(|error| Error::Unprocessable(format!("UTF8 Parse Error: {error}")))?;

        let components: Vec<_> = decoded_text.split(':').collect();
        if components.len() != 2 {
            return Err(Error::Unprocessable("Non-Colon Pair Given".into()));
        }

        let (username, password) = (components[0].trim(), components[1].trim());
        if username.is_empty() || password.is_empty() {
            return Err(Error::Unprocessable("No Credentials Given".into()));
        }

        Ok(Basic {
            username: username.into(),
            password: password.into(),
        })
    }
}

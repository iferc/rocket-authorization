use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Basic {
    pub username: String,
    pub password: String,
}

#[rocket::async_trait]
impl Authorization for Basic {
    const KIND: &'static str = "Basic";

    async fn parse(_: &str, credential: &str, _: &Request) -> Result<Self, ParseError> {
        let decoded_payload = base64::decode(credential).or(Err(
            ParseError::CredentialMalformed(String::from("Base64 Decode Error")),
        ))?;
        let decoded_text = String::from_utf8(decoded_payload).or(Err(
            ParseError::CredentialMalformed(String::from("UTF8 Parse Error")),
        ))?;

        let components: Vec<_> = decoded_text.split(":").collect();
        if components.len() != 2 {
            return Err(ParseError::CredentialMalformed(String::from(
                "Non-Colon Pair Given",
            )));
        }

        let (username, password) = (components[0].trim(), components[1].trim());
        if username.len() == 0 || password.len() == 0 {
            return Err(ParseError::CredentialMalformed(String::from(
                "No Credentials Given",
            )));
        }

        Ok(Basic {
            username: username.into(),
            password: password.into(),
        })
    }
}

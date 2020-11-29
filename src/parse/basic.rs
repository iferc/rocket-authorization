use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Basic {
    pub username: String,
    pub password: String,
}

impl Authorization for Basic {
    const KIND: &'static str = "Basic";
    fn parse(_: &str, credential: &str) -> Result<Self, ParseError> {
        let decoded_payload = base64::decode(credential).or(Err(ParseError::Base64DecodeError))?;
        let decoded_text = String::from_utf8(decoded_payload).or(Err(ParseError::UTFParseError))?;

        let components: Vec<_> = decoded_text.split(":").collect();
        if components.len() != 2 {
            return Err(ParseError::NonColonPairError);
        }

        let (username, password) = (components[0].trim(), components[1].trim());
        if username.len() == 0 || password.len() == 0 {
            return Err(ParseError::EmptyError);
        }

        Ok(Basic {
            username: username.into(),
            password: password.into(),
        })
    }
}

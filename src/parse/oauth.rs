use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OAuth {
    pub token: String,
}

impl Authorization for OAuth {
    const KIND: &'static str = "Bearer";
    fn parse(_: &str, credential: &str) -> Result<Self, ParseError> {
        let decoded_text =
            String::from_utf8(credential.into()).or(Err(ParseError::UTFParseError))?;

        let token_str = decoded_text.trim();
        if token_str.len() == 0 {
            return Err(ParseError::EmptyError);
        }

        Ok(OAuth {
            token: token_str.into(),
        })
    }
}

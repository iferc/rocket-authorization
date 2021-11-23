use rocket_authorization::parse::{Authorization, ParseError, Request};

#[derive(Debug)]
pub struct CustomAuth {
    pub slug: String,
    pub token: String,
}

impl Authorization for CustomAuth {
    const KIND: &'static str = "Custom";

    fn parse(_: &str, credential: &str, _request: &Request) -> Result<Self, ParseError> {
        let components: Vec<_> = credential.split(":").collect();
        if components.len() != 2 {
            return Err(ParseError::CredentialMalformed(String::from(
                "Invalid Key-Value Pair Format Error",
            )));
        }

        let (slug, token) = (components[0].trim(), components[1].trim());
        if slug.len() == 0 || token.len() == 0 {
            return Err(ParseError::HeaderMissing);
        }

        Ok(CustomAuth {
            slug: slug.into(),
            token: token.into(),
        })
    }
}

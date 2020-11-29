use rocket::{http::Status, request::FromRequest, Outcome, Request};

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    NotExists,
    EmptyError,
    UnknownKind,
    InvalidHeader,
    UTFParseError,
    Base64DecodeError,
    NonColonPairError,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AuthorizationCredential {
    BasicCredential { username: String, password: String },
    BearerToken(String),
}

impl AuthorizationCredential {
    fn basic_credentials_parse(credentials: &str) -> Result<AuthorizationCredential, ParseError> {
        let decoded_payload = base64::decode(credentials).or(Err(ParseError::Base64DecodeError))?;
        let decoded_text = String::from_utf8(decoded_payload).or(Err(ParseError::UTFParseError))?;

        let components: Vec<_> = decoded_text.split(":").collect();
        if components.len() != 2 {
            return Err(ParseError::NonColonPairError);
        }

        let (username, password) = (components[0].trim(), components[1].trim());
        if username.len() == 0 || password.len() == 0 {
            return Err(ParseError::EmptyError);
        }

        Ok(AuthorizationCredential::BasicCredential {
            username: username.into(),
            password: password.into(),
        })
    }

    fn bearer_token_parse(credentials: &str) -> Result<AuthorizationCredential, ParseError> {
        let decoded_text =
            String::from_utf8(credentials.into()).or(Err(ParseError::UTFParseError))?;

        let token = decoded_text.trim();
        if token.len() == 0 {
            return Err(ParseError::EmptyError);
        }

        Ok(AuthorizationCredential::BearerToken(token.into()))
    }

    fn parse(authorization_header: &str) -> Result<AuthorizationCredential, ParseError> {
        let header_sections: Vec<_> = authorization_header.split_whitespace().collect();
        if header_sections.len() != 2 {
            return Err(ParseError::InvalidHeader);
        }

        match header_sections[0] {
            "Basic" => Self::basic_credentials_parse(header_sections[1]),
            "Bearer" => Self::bearer_token_parse(header_sections[1]),
            _ => Err(ParseError::UnknownKind),
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthorizationCredential {
    type Error = ParseError;

    fn from_request(
        request: &Request,
    ) -> Outcome<Self, (Status, <Self as FromRequest<'a, 'r>>::Error), ()> {
        match request.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::Unauthorized, ParseError::NotExists)),
            Some(authorization_header) => {
                match AuthorizationCredential::parse(authorization_header) {
                    Ok(credentials) => Outcome::Success(credentials),
                    Err(err) => Outcome::Failure((Status::BadRequest, err)),
                }
            }
        }
    }
}

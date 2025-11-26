use base64::prelude::*;

/// Authentication credentials for EWS
#[derive(Clone)]
pub enum Credentials {
    /// Basic authentication with username and password
    Basic {
        /// Username for authentication
        username: String,
        /// Password for authentication
        password: String,
    },

    /// `OAuth2` authentication with bearer token
    OAuth2 {
        /// `OAuth2` bearer token
        token: String,
    },
}

impl Credentials {
    /// Create Basic authentication credentials
    pub fn basic(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self::Basic {
            username: username.into(),
            password: password.into(),
        }
    }

    /// Create `OAuth2` authentication credentials
    pub fn oauth2(token: impl Into<String>) -> Self {
        Self::OAuth2 { token: token.into() }
    }

    /// Format credentials as HTTP Authorization header value
    pub fn to_auth_header(&self) -> String {
        match self {
            Self::Basic { username, password } => {
                let auth_string = BASE64_STANDARD.encode(format!("{username}:{password}"));
                format!("Basic {auth_string}")
            }
            Self::OAuth2 { token } => {
                format!("Bearer {token}")
            }
        }
    }
}

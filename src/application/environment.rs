use envconfig::{Envconfig, Error as EnvconfigError};
use thiserror::Error;


/// Errors related to environment variable
/// valdiation.
#[derive(Error, Debug)]
pub enum EnvironmentError {
    #[error("Couldn't load the environment, {0:#}")]
    LoadingError(#[from] EnvconfigError)
}

/// This struct loads variables from the .env
/// file and overall machine environment.
///
/// It's meant to store protected variables,
/// they are mostly primitives following the
/// .env convention.
///
/// For more advanced variables that aren't
/// protected see [super::configuration]
#[derive(Envconfig, PartialEq)]
pub struct Environment {
    #[envconfig(from = "DISCORD_TOKEN")]
    discord_token: String,
    #[envconfig(from = "GEMINI_TOKEN")]
    gemini_token: String
}

impl Environment {
    /// Load and validate the environment variables.
    ///
    /// Missing variables will cause this to error.
    pub fn load_validated() -> Result<Self, EnvironmentError> {
        Ok(
            Self::init_from_env()
                .inspect_err(|err| log::error!("{err:#}"))?
        )
    }

    /// The disacord application token, **this should never be displayed**.
    pub fn discord_token(&self) -> &str {
        &self.discord_token
    }

    /// The google gemini AI token.
    pub fn gemini_token(&self) -> &str {
        &self.gemini_token
    }
}

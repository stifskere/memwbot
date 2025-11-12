use std::sync::Arc;

use config::ConfigError;
use gemini_rust::{Gemini, ClientError as GeminiClientError};
use serenity::prelude::TypeMapKey;
use thiserror::Error;

use crate::application::{configuration::Configuration, environment::{Environment, EnvironmentError}};

/// For errors happening while loading [AppContext].
#[derive(Error, Debug)]
pub enum AppContextError {
    #[error("Gemini client error: {0:#}")]
    GeminiInitError(#[from] GeminiClientError),

    #[error("Environment error, {0:#}")]
    EnvironmentError(#[from] EnvironmentError),

    #[error("Configuration error, {0:#}")]
    ConfigError(#[from] ConfigError)
}

/// Key for serenity context store.
pub struct AppContextKey;

/// An amalgamation of application structures
/// such as environment, configuration, third party
/// clients and so on.
pub struct AppContext {
    gemini_client: Gemini,

    configuration: Configuration,
    environment: Environment
}

impl TypeMapKey for AppContextKey {
    type Value = Arc<AppContext>;
}

impl AppContext {
    /// Create a new application context instance,
    /// this will load everything for the application to run.
    ///
    /// This should run only once and be stored in serenity
    /// bot context.
    pub fn create() -> Result<Self, AppContextError> {
        let environment = Environment::load_validated()?;

        Ok(Self {
            gemini_client: Gemini::new(environment.gemini_token())?,

            environment: environment,
            configuration: Configuration::load()?
        })
    }

    /// The google GEMINI client.
    pub fn gemini_client(&self) -> &Gemini {
        &self.gemini_client
    }

    /// The application environment variables.
    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    /// The application non sensible configuration.
    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }
}

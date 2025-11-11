use std::sync::Arc;

use config::ConfigError;
use serenity::prelude::TypeMapKey;
use thiserror::Error;

use crate::application::{configuration::Configuration, environment::{Environment, EnvironmentError}};

/// For errors happening while loading [AppContext].
#[derive(Error, Debug)]
pub enum AppContextError {
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
        Ok(Self {
            environment: Environment::load_validated()?,
            configuration: Configuration::load()?
        })
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

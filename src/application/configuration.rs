use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct ConfigGuildWelcomeInfo {
    welcome_channel: u64,
    welcome_roles: Vec<u64>
}

#[derive(Deserialize, Debug)]
pub struct ConfigGuildInfo {
    guild_id: u64,

    welcome_info: ConfigGuildWelcomeInfo
}

#[derive(Deserialize, Debug)]
pub struct Configuration {
    guild_info: ConfigGuildInfo
}

impl ConfigGuildWelcomeInfo {
    /// The channel used to welcome members.
    pub fn welcome_channel(&self) -> u64 {
        self.welcome_channel
    }

    /// The roles given to the new member.
    pub fn welcome_roles(&self) -> &[u64] {
        &self.welcome_roles
    }
}

impl ConfigGuildInfo {
    /// The guild id this bot will act on.
    pub fn guild_id(&self) -> u64 {
        self.guild_id
    }

    /// Information sources about required
    /// stuff for proper welcoming.
    pub fn welcome_info(&self) -> &ConfigGuildWelcomeInfo {
        &self.welcome_info
    }
}

impl Configuration {
    pub fn load() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name("Config.toml"))
            .add_source(Environment::with_prefix("config"))
            .build()?
            .try_deserialize()
    }

    pub fn guild_info(&self) -> &ConfigGuildInfo {
        &self.guild_info
    }
}

use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;
use serenity::all::{ChannelId, GuildId, RoleId};


#[derive(Deserialize, Debug)]
pub struct ConfigGuildWelcomeInfo {
    welcome_channel: ChannelId,
    welcome_roles: Vec<RoleId>
}

#[derive(Deserialize, Debug)]
pub struct ConfigGuildAIInfo {
    ai_channel: ChannelId
}

#[derive(Deserialize, Debug)]
pub struct ConfigGuildInfo {
    guild_id: GuildId,

    welcome_info: ConfigGuildWelcomeInfo,
    ai_info: ConfigGuildAIInfo
}

#[derive(Deserialize, Debug)]
pub struct Configuration {
    guild_info: ConfigGuildInfo
}

impl ConfigGuildWelcomeInfo {
    /// The channel used to welcome members.
    pub fn welcome_channel(&self) -> ChannelId {
        self.welcome_channel
    }

    /// The roles given to the new member.
    pub fn welcome_roles(&self) -> &[RoleId] {
        &self.welcome_roles
    }
}

impl ConfigGuildAIInfo {
    /// The channel where the bot will be able to chat.
    pub fn ai_channel(&self) -> ChannelId {
        self.ai_channel
    }
}

impl ConfigGuildInfo {
    /// The guild id this bot will act on.
    pub fn guild_id(&self) -> GuildId {
        self.guild_id
    }

    /// Information sources about required
    /// stuff for proper welcoming.
    pub fn welcome_info(&self) -> &ConfigGuildWelcomeInfo {
        &self.welcome_info
    }

    /// Information about how AI should behave.
    pub fn ai_info(&self) -> &ConfigGuildAIInfo {
        &self.ai_info
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

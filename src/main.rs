use std::env::VarError;
use std::sync::Arc;

use dotenvy::dotenv;
use thiserror::Error;
use tokio::main;
use serenity::{Client, Error as SerenityError};
use serenity::all::GatewayIntents;
use tracing_subscriber::{fmt, EnvFilter};

use crate::application::context::{AppContext, AppContextError, AppContextKey};
use crate::events::ai_channel::AiChannelEventHandler;
use crate::events::member_salute::MemberSaluteEventHandler;
use crate::events::shard_presence::ShardPresenceEventHandler;

mod application;
mod events;
mod helpers;


#[derive(Error, Debug)]
enum BotError {
    #[error("Environment variable is missing, {0:#}")]
    MissingEnvironment(#[from] VarError),

    #[error("Serenity error, {0:#}")]
    Serenity(#[from] SerenityError),

    #[error("Error loading context, {0:#}")]
    ContextError(#[from] AppContextError)
}

#[main]
async fn main() -> Result<(), BotError> {
    dotenv().ok();

    let context = AppContext::create()?;

    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mut client = Client::builder(
        context
            .environment()
            .discord_token(),
        GatewayIntents::all()
    )
        .event_handler(AiChannelEventHandler)
        .event_handler(MemberSaluteEventHandler)
        .event_handler(ShardPresenceEventHandler::default())
        .await?;

    {
        let mut client_data = client
            .data
            .write()
            .await;

        client_data
            .insert::<AppContextKey>(Arc::new(context));
    }

    client
        .start()
        .await?;

    Ok(())
}

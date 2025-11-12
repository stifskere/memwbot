use std::error::Error as StdError;

use thiserror::Error;
use serenity::all::*;
use serenity::futures::StreamExt;

use crate::application::context::AppContextKey;


#[derive(Error, Debug)]
enum MessageCreateError {
    #[error("Missing AppContext in application data.")]
    MissingContext,
}

pub struct AiChannelEventHandler;

#[async_trait]
impl EventHandler for AiChannelEventHandler {
    async fn message(&self, context: Context, message: Message) {
        async fn do_message(context: Context, message: Message) -> Result<(), Box<dyn StdError>> {
            if message.author.bot {
                return Ok(());
            }

            let app_data = context
                .data
                .read()
                .await;

            let app_context = app_data
                .get::<AppContextKey>()
                .ok_or(MessageCreateError::MissingContext)?;

            let act_guild_id = app_context
                .configuration()
                .guild_info()
                .guild_id();

            let ai_channel = app_context
                .configuration()
                .guild_info()
                .ai_info()
                .ai_channel();

            if
                message.guild_id.map(|guid_id| guid_id != act_guild_id).unwrap_or(false)
                || message.channel_id != ai_channel 
            {
                return Ok(());
            }

            let message_history = message.channel_id
                .messages_iter(&context.http)
                .take(10)
                .skip(1)
                .collect::<Vec<_>>()
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|message| format!(
                    "{} {} => \"{}\";\n",
                    message.author.display_name(),
                    message.author.id
                        .eq(&context.cache.current_user().id)
                        .then_some("(you)")
                        .unwrap_or(""),
                    message.content
                ))
                .rev()
                .collect::<String>();

            let ai_context = include_str!("ai_channel_context.txt")
                .replace("{message_history}", &message_history);

            let ai_response = app_context
                .gemini_client()
                .generate_content()
                .with_system_instruction(ai_context)
                .with_user_message(&message.content)
                .execute()
                .await?
                .text();

            if ai_response.is_empty() {
                return Ok(());
            }

            message
                .channel(&context.http)
                .await?
                .id()
                .send_message(
                    &context.http,
                    CreateMessage::new()
                        .content(ai_response)
                )
                .await?;

            Ok(())
        }

        if let Err(error) = do_message(context, message).await {
            log::error!("{error:#}");
        };
    }
}

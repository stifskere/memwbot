use std::error::Error as StdError;

use serenity::all::*;
use thiserror::Error;

use crate::application::context::AppContextKey;


#[derive(Error, Debug)]
enum GuildMemberAddError {
    #[error("Missing AppContext in application data.")]
    MissingContext,

    #[error("The welcome channel is missing.")]
    MissingWelcomeChannel
}

pub struct MemberSaluteEventHandler;

#[async_trait]
impl EventHandler for MemberSaluteEventHandler {
    async fn guild_member_addition(&self, context: Context, member: Member) {
        async fn do_guild_member_addition(context: Context, member: Member) -> Result<(), Box<dyn StdError>> {
            let app_data = context
                .data
                .read()
                .await;

            let app_context = app_data
                .get::<AppContextKey>()
                .ok_or(GuildMemberAddError::MissingContext)?;

            let act_guild_id = app_context
                .configuration()
                .guild_info()
                .guild_id();

            let welcome_channel = app_context
                .configuration()
                .guild_info()
                .welcome_info()
                .welcome_channel();

            let welcome_roles = app_context
                .configuration()
                .guild_info()
                .welcome_info()
                .welcome_roles();

            if member.guild_id != act_guild_id {
                return Ok(());
            }

            let guild_channels = member
                .guild_id
                .channels(&context.http)
                .await?;
            let welcome_channel = guild_channels
                .get(&welcome_channel)
                .ok_or(GuildMemberAddError::MissingWelcomeChannel)?;

            let welcome_message = CreateMessage::new()
                .content(member.mention().to_string().repeat(10))
                .add_embed(
                    CreateEmbed::new()
                        .title(format!("Welcome {}!", member.display_name()))
                        .description(concat!(
                            "I'm lost, so I can't really welcome you properly.",
                            " Where are we? Do you know anything about this?",
                            " **сука блять я теряюсь где нахожусь, это не матушка россия,",
                            " я не могу трахать русских в задницу**"
                        ))
                        .image("https://i.pinimg.com/236x/50/21/12/50211206cbc56b59d1de4cf90145f0ee.jpg")
                        .color(0x013220)
                        .footer(CreateEmbedFooter::new("Insert your footer here."))
                );

            welcome_channel.send_message(&context.http, welcome_message)
                .await?;

            member.add_roles(&context.http, welcome_roles).await?;

            log::info!("{} Joined {}", member.display_name(), act_guild_id.to_string());

            Ok(())
        }

        if let Err(error) = do_guild_member_addition(context, member).await {
            log::error!("{error:#}");
        }
    }
}

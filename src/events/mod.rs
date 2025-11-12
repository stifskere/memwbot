use serenity::async_trait;
use serenity::all::*;

pub mod guild_member_add;
pub mod message_create;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, context: Context, member: Member) {
        if let Err(error) = guild_member_add::guild_member_add(context, member).await {
            log::error!("{error:#}");
        }
    }

    async fn message(&self, context: Context, message: Message) {
        if let Err(error) = message_create::message_create(context, message).await {
            log::error!("{error:#}");
        }
    }
}

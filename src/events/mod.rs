use serenity::async_trait;
use serenity::all::*;

pub mod guild_member_add;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, context: Context, member: Member) {
        if let Err(error) = guild_member_add::guild_member_add(context, member).await {
            log::error!("{error:#}");
        }
    }
}

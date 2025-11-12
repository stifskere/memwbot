use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

use serenity::all::*;
use tokio::time::sleep;
use tokio::spawn;


#[derive(Default)]
pub struct ShardPresenceEventHandler {
    is_ready: AtomicBool
}

#[async_trait]
impl EventHandler for ShardPresenceEventHandler {
    async fn ready(&self, context: Context, _ready: Ready) {
        {
            if self.is_ready.load(Ordering::SeqCst) {
                return;
            }

            self.is_ready.store(true, Ordering::SeqCst);
        }

        spawn(async move {
            let presences = &[
                (ActivityData::playing("with your feelings"), OnlineStatus::Online),
                (ActivityData::listening("trough your walls"), OnlineStatus::Online),
                (ActivityData::watching("trough your window"), OnlineStatus::Online)
            ];

            // what a time to be alive.
            let mut presence_index = 0usize;

            loop {
                let presence = &presences[presence_index];
                presence_index = (presence_index + 1) % presences.len();

                context.set_presence(
                    Some(presence.0.clone()),
                    presence.1.clone()
                );

                sleep(Duration::from_secs(const { 1 * 60 * 60 })).await;
            }
        });
    }
}

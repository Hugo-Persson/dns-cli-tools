use crate::discord_webhook::DiscordWebhook;
use serde::{Deserialize, Serialize};

pub trait WebhookNotifier {
    async fn change_ip(&self, from: &String, to: &String) -> ();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WebhookNotifierType {
    DiscordWebhook(DiscordWebhook),
}


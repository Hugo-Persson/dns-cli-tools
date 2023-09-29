use serde::{Deserialize, Serialize};
use crate::discord_webhook::DiscordWebhook;


pub trait WebhookNotifier{
    async fn change_ip(&self, from: &String, to: &String) -> ();
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WebhookNotifierType{
    DiscordWebhook(DiscordWebhook),
}
use crate::webhook_notifier::WebhookNotifier;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscordWebhook {
    url: String,
}

impl DiscordWebhook {
    pub fn new(url: String) -> DiscordWebhook {
        DiscordWebhook { url }
    }
}

impl WebhookNotifier for DiscordWebhook {
    async fn change_ip(&self, from: &String, to: &String) {
        println!("DiscordWebhook::change_ip from: {}, to: {}", from, to);
        let client = Client::new();
        let _ = client
            .post(&self.url)
            .json(&json!({
                "content": format!("IP changed from {} to {}", from, to)
            }))
            .send()
            .await;
    }
}

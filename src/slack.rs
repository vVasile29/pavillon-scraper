use crate::domain::PavillonDishes;
use dotenv::dotenv;
use num_format::{Locale, ToFormattedString};
use reqwest::Url;
use slack_morphism::prelude::*;
use std::env;

const SLACK_CHANNEL: &str = "#pavillon-test";

pub async fn post_pavillon_dishes_to_slack(dishes: PavillonDishes) {
    dotenv().ok();
    send_message(
        SLACK_CHANNEL,
        PavillonMessage::from(dishes).render_template(),
    )
    .await
    .expect("TODO: panic message");
}

async fn send_message<S: Into<SlackChannelId>>(
    channel: S,
    message: SlackMessageContent,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let bot_token = env::var("TOKEN").expect("TOKEN env var must be set");
    let client = SlackClient::new(SlackClientHyperConnector::new()?);

    // Create our Slack API token
    let token: SlackApiToken = SlackApiToken::new(bot_token.into());

    // Create a Slack session with this token
    // A session is just a lightweight wrapper around your token
    // not to specify it all the time for series of calls.
    let session = client.open_session(&token);

    // Send a text message
    let post_chat_req = SlackApiChatPostMessageRequest::new(channel.into(), message);

    let _post_chat_resp = session.chat_post_message(&post_chat_req).await?;

    Ok(())
}

#[derive(Debug)]
struct PavillonMessage(PavillonDishes);

impl From<PavillonDishes> for PavillonMessage {
    fn from(value: PavillonDishes) -> Self {
        Self(value)
    }
}

impl SlackMessageTemplate for PavillonMessage {
    fn render_template(&self) -> SlackMessageContent {
        let mut slack_blocks: Vec<SlackBlock> = self
            .0
            .dishes
            .iter()
            .map(|dish| {
                SlackSectionBlock::new()
                    .with_text(md!(
                        "*{}€* {}",
                        format!("{:.2}", dish.price).replace('.', ","),
                        dish.name
                    ))
                    .into()
            })
            .collect();

        slack_blocks.push(
            SlackActionsBlock::new(slack_blocks![some_into(
                SlackBlockButtonElement::new("simple-message-button".into(), pt!("Karte öffnen"))
                    .with_url(self.0.url.clone())
            )])
            .into(),
        );
        SlackMessageContent::new().with_blocks(slack_blocks)
    }
}

use dotenv::dotenv;
use slack_morphism::prelude::*;
use std::env;

const SLACK_CHANNEL: &str = "#pavillon-test";

pub async fn main() {
    dotenv().ok();
    send_message(SLACK_CHANNEL, "Hello World")
        .await
        .expect("TODO: panic message");
}

async fn send_message<S: Into<SlackChannelId>, M: Into<String>>(
    channel: S,
    message: M,
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
    let post_chat_req = SlackApiChatPostMessageRequest::new(
        channel.into(),
        SlackMessageContent::new().with_text(message.into()),
    );

    let _post_chat_resp = session.chat_post_message(&post_chat_req).await?;

    Ok(())
}

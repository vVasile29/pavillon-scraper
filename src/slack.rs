use dotenv::dotenv;
use slack_morphism::prelude::*;
use std::env;

pub async fn main() {
    dotenv().ok();
    send_message("Hello World", "#pavillion-test")
        .await
        .expect("TODO: panic message");
}

async fn send_message(
    message: &str,
    channel: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let bot_token = env::var("TOKEN").expect("TOKEN env var musst be set");
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

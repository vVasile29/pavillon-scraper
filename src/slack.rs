use crate::domain::PavillonDishes;
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use slack_morphism::errors::SlackClientError;
use slack_morphism::prelude::*;
use std::env;
use std::error::Error;

const SLACK_CHANNEL: &str = "#pavillon-test";

type Connector = SlackClientHyperConnector<HttpsConnector<HttpConnector>>;

pub struct SlackApi {
    client: SlackClient<Connector>,
    token: SlackApiToken,
}

impl SlackApi {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        dotenv::dotenv()?;
        let client = SlackClient::new(SlackClientHyperConnector::new()?);

        let bot_token = env::var("TOKEN").expect("TOKEN env var must be set");
        let token = SlackApiToken::new(bot_token.into());

        Ok(SlackApi { client, token })
    }

    fn open_session(&self) -> SlackClientSession<Connector> {
        self.client.open_session(&self.token)
    }

    async fn send_message<S: Into<SlackChannelId>>(
        &self,
        channel: S,
        message: SlackMessageContent,
    ) -> Result<(), SlackClientError> {
        let post_chat_req = SlackApiChatPostMessageRequest::new(channel.into(), message);
        self.open_session()
            .chat_post_message(&post_chat_req)
            .await
            .map(|_| ())
    }

    pub async fn post_pavillon_dishes_to_slack(
        &self,
        dishes: PavillonDishes,
    ) -> Result<(), SlackClientError> {
        self.send_message(
            SLACK_CHANNEL,
            PavillonMessage::from(dishes).render_template(),
        )
        .await
    }
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

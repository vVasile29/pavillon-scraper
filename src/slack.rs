use crate::domain::PavillonDishes;
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use slack_morphism::errors::SlackClientError;
use slack_morphism::prelude::*;
use std::error::Error;
use std::path::Path;
use std::{env, fs};

const SLACK_CHANNEL: &str = "#pavillon-test";
const SLACK_CHANNEL_ID: &str = "C0762G6JUDP";

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

    async fn send_message<C: Into<SlackChannelId>>(
        &self,
        channel: C,
        message: SlackMessageContent,
    ) -> Result<(), SlackClientError> {
        let post_chat_req = SlackApiChatPostMessageRequest::new(channel.into(), message);
        self.open_session()
            .chat_post_message(&post_chat_req)
            .await
            .map(|_| ())
    }

    pub async fn upload_file<C: Into<SlackChannelId>, P: AsRef<Path>>(
        &self,
        channel: C,
        path: P,
    ) -> Result<(), SlackClientError> {
        let session = self.open_session();

        let len = path.as_ref().metadata().unwrap().len();
        let upload_url_resp = session
            .get_upload_url_external(&SlackApiFilesGetUploadUrlExternalRequest::new(
                "test.pdf".to_string(),
                len as usize,
            ))
            .await
            .unwrap();

        let file_upload_resp = session
            .files_upload_via_url(&SlackApiFilesUploadViaUrlRequest {
                upload_url: upload_url_resp.upload_url,
                content: fs::read(path).unwrap(),
                content_type: "application/pdf".into(),
            })
            .await
            .unwrap();

        let complete_file_upload_req =
            SlackApiFilesCompleteUploadExternalRequest::new(vec![SlackApiFilesComplete::new(
                upload_url_resp.file_id,
            )])
            .with_channel_id(SLACK_CHANNEL_ID.into());
        let complete_file_upload_resp = session
            .files_complete_upload_external(&complete_file_upload_req)
            .await
            .unwrap();
        println!("{:?}", complete_file_upload_resp);
        Ok(())
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

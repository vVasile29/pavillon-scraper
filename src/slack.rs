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

    pub async fn test_send_message<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let resp = self.upload_file(path).await.unwrap();
        let SlackFile {
            url_private,
            url_private_download,
            permalink,
            permalink_public,
            ..
        } = &resp.files[0];
        let string = format!(
            "Linktest permalink_public {}",
            permalink_public.as_ref().unwrap()
        );
        self.send_message(SLACK_CHANNEL, SlackMessageContent::new().with_text(string))
            .await
            .unwrap();
        Ok(())
    }

    pub async fn upload_file<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<SlackApiFilesCompleteUploadExternalResponse, SlackClientError> {
        let session = self.open_session();

        let len = path.as_ref().metadata().unwrap().len();
        let upload_url_resp = session
            .get_upload_url_external(&SlackApiFilesGetUploadUrlExternalRequest::new(
                "test.pdf".to_string(),
                len as usize,
            ))
            .await?;

        let _file_upload_resp = session
            .files_upload_via_url(&SlackApiFilesUploadViaUrlRequest {
                upload_url: upload_url_resp.upload_url,
                content: fs::read(path).unwrap(),
                content_type: "application/pdf".into(),
            })
            .await?;

        let complete_file_upload_req =
            SlackApiFilesCompleteUploadExternalRequest::new(vec![SlackApiFilesComplete::new(
                upload_url_resp.file_id,
            )]);
        let complete_file_upload_resp = session
            .files_complete_upload_external(&complete_file_upload_req)
            .await?;
        Ok(complete_file_upload_resp)
    }

    pub async fn post_pavillon_dishes_to_slack(
        &self,
        dishes: PavillonDishes,
    ) -> Result<(), SlackClientError> {
        let files = self.upload_file(&dishes.path).await.unwrap().files;
        self.send_message(
            SLACK_CHANNEL,
            PavillonMessage(dishes, files.into_iter().nth(0).unwrap()).render_template(),
        )
        .await
    }
}

#[derive(Debug)]
struct PavillonMessage(PavillonDishes, SlackFile);

impl SlackMessageTemplate for PavillonMessage {
    fn render_template(&self) -> SlackMessageContent {
        let Self(pavillon_dishes, slack_file) = self;
        let mut slack_blocks: Vec<SlackBlock> = vec![];

        let side_dishes = pavillon_dishes.available_side_dishes();
        if !side_dishes.is_empty() {
            slack_blocks.push(
                SlackSectionBlock::new()
                    .with_text(md!(
                        "Heute gibt es {}!",
                        side_dishes
                            .iter()
                            .map(|dish| format!(
                                "{}{}",
                                dish.colloquial_name,
                                dish.emoji
                                    .map(|emoji| format!(" {}", emoji))
                                    .unwrap_or_default()
                            ))
                            .collect::<Vec<_>>()
                            .join(" und ")
                    ))
                    .into(),
            )
        }

        slack_blocks.extend(pavillon_dishes.dishes.iter().map(|dish| {
            SlackSectionBlock::new()
                .with_text(md!(
                    "*{}€* {}",
                    format!("{:.2}", dish.price).replace('.', ","),
                    dish.name
                ))
                .into()
        }));

        slack_blocks.push(
            SlackActionsBlock::new(slack_blocks![some_into(
                SlackBlockButtonElement::new("simple-message-button".into(), pt!("Karte öffnen"))
                    .with_url(self.0.url.clone())
            )])
            .into(),
        );
        slack_blocks.push(
            SlackSectionBlock::new()
                .with_text(pt!("{}", slack_file.permalink.as_ref().unwrap()))
                .into(),
        );
        SlackMessageContent::new().with_blocks(slack_blocks)
    }
}

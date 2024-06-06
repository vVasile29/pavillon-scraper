use crate::domain::{PavillonDish, PavillonDishes};

mod domain;
mod input;
mod pdf_parser;
mod slack;

#[tokio::main]
async fn main() {
    let (url, pdf) = input::download_pdf().await;
    let dishes: Vec<PavillonDish> = pdf_parser::parse_pdf(&pdf).expect("TODO: panic message");
    let slack_api = slack::SlackApi::new().unwrap();

    let dishes = PavillonDishes {
        url: url.parse().unwrap(),
        path: pdf.path().into(),
        dishes,
    };

    slack_api
        .post_pavillon_dishes_to_slack(dishes)
        .await
        .unwrap();
}

use crate::domain::{PavillonDish, PavillonDishes};

mod domain;
mod input;
mod pdf_parser;
mod slack;

#[tokio::main]
async fn main() {
    let (url, pdf) = input::download_pdf().await;
    let dishes: Vec<PavillonDish> = pdf_parser::parse_pdf(&pdf).expect("TODO: panic message");

    slack::post_pavillon_dishes_to_slack(PavillonDishes {
        url: url.parse().unwrap(),
        dishes,
    })
    .await;
}

use crate::domain::{PavillonDish, PavillonDishes};

mod domain;
mod input;
mod pdf_parser;
mod slack;

#[tokio::main]
async fn main() {
    let pdf = input::download_pdf().await;
    let dishes: Vec<PavillonDish> = pdf_parser::parse_pdf(&pdf).expect("TODO: panic message");
    let pdf_url: String = input::get_pdf_link().await;

    slack::post_pavillon_dishes_to_slack(get_pavillon_dishes(dishes, pdf_url)).await;
}

fn get_pavillon_dishes(dishes: Vec<PavillonDish>, pdf_url: String) -> PavillonDishes {
    PavillonDishes {
        url: pdf_url.parse().unwrap(),
        dishes,
    }
}

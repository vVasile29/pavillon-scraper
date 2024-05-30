use crate::domain::{PavillonDish, PavillonDishes};

mod domain;
mod input;
mod pdf_parser;
mod slack;

#[tokio::main]
async fn main() {
    let pdf = input::download_pdf().await;
    pdf_parser::parse_pdf(&pdf).expect("TODO: panic message");
    slack::post_pavillon_dishes_to_slack(example_dishes()).await;
}

fn example_dishes() -> PavillonDishes {
    PavillonDishes {
        url: "https://www.pavillon-wuerzburg.de/wp/wp-content/uploads/2024/05/Tageskarte-31.05.2024.pdf".try_into().unwrap(),
        dishes: vec![
            PavillonDish {
                name: "Gericht 1".to_string(),
                price: 4.5,
            },
            PavillonDish {
                name: "Gericht 2".to_string(),
                price: 10.,
            },
            PavillonDish {
                name: "Kürbishummer mit Kaviar".to_string(),
                price: 99.95,
            },
        ],
    }
}

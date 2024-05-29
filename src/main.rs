use crate::pdf_parser::parse_pdf;

mod input;
mod slack;
mod pdf_parser;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    parse_pdf().expect("TODO: panic message");
    slack::main().await;
}

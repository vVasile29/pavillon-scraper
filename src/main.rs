mod domain;
mod input;
mod pdf_parser;
mod slack;

#[tokio::main]
async fn main() {
    let pdf = input::download_pdf().await;
    pdf_parser::parse_pdf(&pdf).expect("TODO: panic message");
}

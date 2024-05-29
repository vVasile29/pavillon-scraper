use crate::pdf_parser::parse_pdf;

mod input;
mod pdf_parser;

fn main() {
    parse_pdf().expect("TODO: panic message");
}

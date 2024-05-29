use scraper::{Html, Selector};
use std::io::Write;
use tempfile::NamedTempFile;

const PAVILLON_SITE: &str = "https://www.pavillon-wuerzburg.de/pavillon/mittag";

pub fn download_pdf() -> NamedTempFile {
    let pdf_link = get_pdf_link();
    let mut tmp_file = tempfile::Builder::new()
        .prefix("pavillon")
        .tempfile()
        .unwrap();
    let response = reqwest::blocking::get(pdf_link).unwrap();

    tmp_file.write_all(&response.bytes().unwrap()).unwrap();
    tmp_file
}

fn get_pdf_link() -> String {
    let string = reqwest::blocking::get(PAVILLON_SITE)
        .unwrap()
        .text()
        .unwrap();
    let html = Html::parse_document(&string);

    let all_pdf_a_tags = Selector::parse(
        r#"a[href^="https://www.pavillon-wuerzburg.de/wp/wp-content/uploads/"][href$=".pdf"]"#,
    )
    .unwrap();

    // will likely be the Tageskarte, Sandwichauswahl and Getränkekarte
    let all_pdf_links: Vec<&str> = html
        .select(&all_pdf_a_tags)
        .flat_map(|e| e.value().attr("href"))
        .collect();

    all_pdf_links
        .into_iter()
        .find(|link| link.to_lowercase().contains("tageskarte"))
        .unwrap()
        .to_string()
}

use scraper::{Html, Selector};
use std::path::PathBuf;

const PAVILLON_SITE: &'static str = "https://www.pavillon-wuerzburg.de/pavillon/mittag";

pub fn download_pdf() -> PathBuf {
    unimplemented!();
}

pub fn get_pdf_link() -> String {
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

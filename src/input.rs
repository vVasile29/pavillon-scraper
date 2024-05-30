use futures::StreamExt;
use scraper::{Html, Selector};
use tempfile::NamedTempFile;

const PAVILLON_SITE: &str = "https://www.pavillon-wuerzburg.de/pavillon/mittag";

pub async fn download_pdf() -> (String, NamedTempFile) {
    let pdf_link = get_pdf_link().await;
    let tmp_file = tempfile::Builder::new()
        .prefix("pavillon")
        .tempfile()
        .unwrap();

    println!("Downloading {} to {}", pdf_link, tmp_file.path().display());

    let response = reqwest::get(&pdf_link).await.unwrap();

    let mut reader = response.bytes_stream();
    let mut writer = tokio::fs::File::create(&tmp_file).await.unwrap();
    while let Some(item) = reader.next().await {
        tokio::io::copy(&mut item.unwrap().as_ref(), &mut writer)
            .await
            .unwrap();
    }
    (pdf_link, tmp_file)
}

async fn get_pdf_link() -> String {
    let html_content = reqwest::get(PAVILLON_SITE)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let html = Html::parse_document(&html_content);

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

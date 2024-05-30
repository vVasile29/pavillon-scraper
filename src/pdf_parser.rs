use pdf_extract::extract_text;
use regex::Regex;
use std::error::Error;
use std::path::Path;
use crate::domain::{PavillonDish, PavillonDishes};

pub fn parse_pdf<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    // Extract text from the PDF
    let text = extract_text(path)?;

    // Print extracted text for debugging
    println!("{}", text);

    // Initialize a vector to hold menu items
    let mut dishes = Vec::new();

    // Split the text into lines and remove the header and footer
    let lines: Vec<&str> = text.lines()
        .skip_while(|line| !line.trim().is_empty())  // Skip until the first empty line
        .skip(1)  // Skip the empty line itself
        .take_while(|line| !line.contains("Änderungen vorbehalten!"))
        .collect();

    // Rejoin the relevant lines into a single string
    let relevant_text = lines.join("\n");

    // Define a regex to match menu items and prices
    let re = Regex::new(r"(?m)^\s*([^€\n]+(?:\n[^€\n]+)*)\s+€\s*([0-9]+,[0-9]+)\s*$").unwrap();

    // Iterate over all matches
    for cap in re.captures_iter(&relevant_text) {
        let mut name = cap[1].trim().replace("\n", " ");
        name = Regex::new(r"\s{2,}").unwrap().replace_all(&name, " ").to_string(); // Replace multiple spaces with a single space
        name = name.replace(" .", "").replace("..", "").trim().to_string();
        name = name.replace("- ", "-"); // Handle the specific case of hyphen followed by space
        let price_str = &cap[2];
        let price = price_str.replace(",", ".").parse::<f32>()?;

        // Create a PavillonDish and add it to the vector
        dishes.push(PavillonDish { name, price });
    }

    // Print the dishes
    for dish in &dishes {
        println!("{:?}", dish);
    }

    Ok(())
}

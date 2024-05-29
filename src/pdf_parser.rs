use std::error::Error;
use std::path::Path;

pub fn parse_pdf<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let string = pdf_extract::extract_text(path).unwrap();

    println!("{}", string);

    Ok(())
}

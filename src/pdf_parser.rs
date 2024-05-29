use pdf::{content::Op, file::FileOptions};
use std::error::Error;
use std::path::Path;

pub fn parse_pdf<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    // Open the PDF file
    let file = FileOptions::cached().open(path)?;

    // Iterate through all pages and extract text content
    for i in 0..file.num_pages() {
        let page = file.get_page(i)?;
        if let Some(content) = page.contents.as_ref() {
            let ops = content.operations(&file.resolver())?;
            println!("Page {}:", i + 1);
            for op in ops {
                match op {
                    Op::TextDraw { ref text } => {
                        println!("TextDraw: {}", text.to_string_lossy());
                    }
                    _ => {
                        println!("{:?}", op); // Print all operations
                    }
                }
            }
        }
    }

    Ok(())
}

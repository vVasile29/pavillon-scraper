use std::path::PathBuf;
use std::error::Error;
use pdf::{
    file::FileOptions,
    content::Op,
};

pub fn parse_pdf() -> Result<(), Box<dyn Error>> {
    // Specify the path to the input PDF file
    let input_path = PathBuf::from("/home/vvasile/Development/pavillon-scraper/assets/sample - sample.pdf");

    // Open the PDF file
    let file = FileOptions::cached().open(&input_path)?;

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
                    },
                    _ => {
                        println!("{:?}", op);  // Print all operations
                    }
                }
            }
        }
    }

    Ok(())
}

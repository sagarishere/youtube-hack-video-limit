use scraper::{Html, Selector};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("./doc.html");

    if !path.exists() {
        eprintln!("File does not exist at the specified path.");
        return;
    }

    let mut file = File::open(&path).expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file");

    let document = Html::parse_document(&contents);

    let h2_selector = Selector::parse("h2[class*=\"heading\"]").expect("Could not create selector");

    // Find all h2 elements that contain '(' and ')'
    let h2_elements: Vec<_> = document
        .select(&h2_selector)
        .filter_map(|el| el.text().next().map(|text| text.trim().to_string().split("(").collect::<Vec<_>>()[0].to_string()))
        .map(|text| {
            // Remove everything after the space and '('
            if let Some(index) = text.find(" (") {
                text[..index].to_string()
            } else {
                text
            }
        })
        .collect();

    // Enumerate and collect the renamed h2 elements
    let enumerated_h2_elements: Vec<String> = h2_elements
        .iter()
        .enumerate()
        .map(|(index, h2)| format!("{:02} {}", index + 1, h2))
        .collect();

    // Print and create directories for each enumerated h2 element
    for h2 in &enumerated_h2_elements {
        // Create directory with the name of the enumerated h2 element
        fs::create_dir(h2).expect("Could not create directory");
    }
}

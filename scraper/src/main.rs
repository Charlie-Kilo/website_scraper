use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::fs::OpenOptions;
use std::io::{Write};
use chrono::{Utc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch the web page
    let url = "https://www.buffalotracedistillery.com/visit-us/tasting-and-purchasing/product-availability.html";
    let body = get(url)?.text()?;
    let document = Html::parse_document(&body);
    
    // Select parent elements with class "container--product-availability-available"
    let available_parent_selector = Selector::parse(".container--product-availability-available").unwrap();
    let mut titles = Vec::new();
    for parent_element in document.select(&available_parent_selector) {
        // Within each parent element, select elements with class "cmp-title__link"
        let title_selector = Selector::parse(".cmp-title__link").unwrap();
        for title_element in parent_element.select(&title_selector) {
            titles.push(title_element.text().collect::<Vec<_>>().join(""));
        }
    }
    
    // Open or create CSV file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("buffalo_trace_products_available.csv")?;
    
    // Write header row if file is empty
    if file.metadata()?.len() == 0 {
        write!(file, "Date & Time Ran,")?; // Write header for date and time column
        // Write header for product columns (up to 9)
        for i in 1..=9 {
            write!(file, "Product {},", i)?;
        }
        writeln!(file)?;
    }
    
    // Get current date and time
    let current_time = Utc::now();
    // Write date and time in the first column
    write!(file, "{},", current_time.format("%Y-%m-%d %H:%M:%S"))?;
    
    // Write titles of available products (up to 9)
    for i in 0..9 {
        if let Some(title) = titles.get(i) {
            write!(file, "{},", title)?;
        } else {
            // If no product is available for this column, print an empty field
            write!(file, ",")?;
        }
    }
    writeln!(file)?;
    
    Ok(())
}

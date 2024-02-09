extern crate reqwest;
extern crate scraper;

use reqwest::blocking::Client;
use scraper::{Html, Selector};

fn fetch_emails(url: &str) -> Result<Vec<String>, reqwest::Error> {
    let body = Client::new().get(url).send()?.text()?;
    let document = Html::parse_document(&body);
    let email_selector = Selector::parse("a[href^=mailto]").unwrap();
    let text_selector = Selector::parse("p, h1, h2, h3, h4, h5, h6, li, td").unwrap();

    let mut emails = Vec::new();

    // Find emails in mailto links
    for element in document.select(&email_selector) {
        if let Some(email) = element.value().attr("href") {
            if let Some(address) = email.strip_prefix("mailto:") {
                emails.push(address.to_string());
            }
        }
    }

    // Find emails in text content
    for element in document.select(&text_selector) {
        let text = element.text().collect::<String>();
        let extracted_emails = extract_emails_from_text(&text);
        emails.extend(extracted_emails);
    }

    Ok(emails)
}

fn extract_emails_from_text(text: &str) -> Vec<String> {
    // Very basic email extraction regex, can be improved for better accuracy
    let re = regex::Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
    let mut emails = Vec::new();
    for mat in re.find_iter(text) {
        emails.push(mat.as_str().to_string());
    }
    emails
}

fn main() {
    println!(
        r#"
 ░▒▓███████▓▒░░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓███████▓▒░░▒▓████████▓▒░▒▓██████████████▓▒░ ░▒▓██████▓▒░░▒▓█▓▒░▒▓█▓▒░        
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░▒▓█▓▒░        
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░▒▓█▓▒░        
░▒▓███████▓▒░░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓██████▓▒░ ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓████████▓▒░▒▓█▓▒░▒▓█▓▒░        
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░▒▓█▓▒░        
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░▒▓█▓▒░        
░▒▓█▓▒░       ░▒▓█████████████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓████████▓▒░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░▒▓████████▓▒░ 
                                                                                by : @Fr3y0                                                                                

"#
    );

    println!("Welcome to Pwnemail - Your Email Finder!");

    // Ask for user input
    println!("Please enter the URL of the website:");
    let mut url = String::new();
    std::io::stdin().read_line(&mut url).expect("Failed to read URL");

    // Trim whitespace and newlines from user input
    let url = url.trim();

    // Fetch emails
    match fetch_emails(url) {
        Ok(emails) => {
            println!("Here are the emails found on '{}': ", url);
            for email in emails {
                println!("{}", email);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("Thank you for using Pwnemail - Your Email Finder!");
}


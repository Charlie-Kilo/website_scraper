use lettre::{message::{SinglePart}, Transport};
use lettre::transport::smtp::authentication::Credentials;
use std::{fs, error::Error};
use csv::ReaderBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure SMTP server details
    let smtp_server = "smtp-mail.com";
    let smtp_port = 587; // Example port, adjust as per your SMTP provider's configuration
    let smtp_username = "enail@example.com";
    let smtp_password = "password";

    // Create SMTP client
    let email_transport = lettre::SmtpTransport::starttls_relay(smtp_server)?
        .port(smtp_port)
        .credentials(Credentials::new(
            smtp_username.to_string(),
            smtp_password.to_string(),
        ))
        .build();

    // Read attachment file (CSV)
    let csv_content = fs::read_to_string("buffalo_trace_products_available.csv")?;

    // Parse CSV content
    let mut rdr = ReaderBuilder::new().from_reader(csv_content.as_bytes());
        
    // Extract headers
    let headers = rdr.headers()?.clone();
    let headers_row = headers.iter()
        .map(|header| format!("<th>{}</th>", header))
        .collect::<Vec<String>>()
        .join("");

    // Prepare email body with HTML formatting
    let mut body_content = String::new();
    for result in rdr.records() {
        let record = result?;
        let record_row = record.iter()
            .map(|field| format!("<td>{}</td>", field))
            .collect::<Vec<String>>()
            .join("");
        body_content.push_str(&format!("<tr>{}</tr>", record_row));
    }
    let email_body = format!(
        r#"
        <h1 style="font-weight: bold; font-size: large;">🐃 Buffalo Trace Tracker 🐃</h1>
        <table border="1">
            <thead>{}</thead>
            <tbody>{}</tbody>
        </table>
        "#,
        headers_row, body_content
    );

    // Create email message
    let email_builder = lettre::Message::builder()
        .from("buffalo_trace_tracker@outlook.com".parse().unwrap());
    let email = email_builder
        .to("dmc1997@outlook.com".parse().unwrap())
        .to("dmc3csc@gmail.com".parse().unwrap())
        .subject("Buffalo Trace Available Product Update")
        .singlepart(
            SinglePart::builder()
                .header(lettre::message::header::ContentType::parse("text/html; charset=utf-8").unwrap())
                .body(email_body),
        )
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // Send email
    email_transport.send(&email)?;
    println!("Email sent successfully!");
    Ok(())
}


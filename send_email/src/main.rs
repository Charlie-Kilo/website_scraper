use lettre::{message::SinglePart, Transport};
use lettre::transport::smtp::authentication::Credentials;
use std::{fs, error::Error};
use csv::{ReaderBuilder, StringRecord};
use chrono::{NaiveDateTime, Utc};

fn main() -> Result<(), Box<dyn Error>> {
    // Configure SMTP server details
    let smtp_server = "smtp-mail.com";
    let smtp_port = 587; // Example port, adjust as per your SMTP provider's configuration
    let smtp_username = "sender@example.com";
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
        .map(|header| format!("<th style=\"padding: 8px;\">{}</th>", header))
        .collect::<Vec<String>>()
        .join("");

    // Extract records, convert each StringRecord into a Vec<String>, and sort by date
    let mut records: Vec<Vec<String>> = rdr.records()
        .map(|r| r.unwrap().iter().map(|f| f.to_string()).collect())
        .collect();
    records.sort_by(|a, b| {
        let date_a = NaiveDateTime::parse_from_str(&a[0], "%Y-%m-%d %H:%M:%S").unwrap();
        let date_b = NaiveDateTime::parse_from_str(&b[0], "%Y-%m-%d %H:%M:%S").unwrap();
        date_b.cmp(&date_a)
    });

    // Prepare HTML table rows with appropriate styling
    let mut body_content = String::new();
    for (i, record) in records.iter().enumerate() {
        let mut record_row = String::new();
        let row_style = if i == 0 { "background-color: lightgreen;" } else { "" };
        for field in record.iter() {
            let cell_content = if field.is_empty() {
                "<td style=\"padding: 8px; color: red;\">❌❌❌</td>".to_string()
            } else {
                format!("<td style=\"padding: 8px;\">{}</td>", field)
            };
            record_row.push_str(&cell_content);
        }
        body_content.push_str(&format!("<tr style=\"{}\">{}</tr>", row_style, record_row));
    }

    let email_body = format!(
        r#"
        <center>
            <h1 style="font-weight: bold; font-size: large; padding: 8px;">🐃 Buffalo Trace Tracker 🐃</h1>
        </center>
        <table border="1" cellspacing="0" cellpadding="8" style="margin: auto;">
            <thead>{}</thead>
            <tbody>{}</tbody>
        </table>
        "#,
        headers_row, body_content
    );

    // Create email message
    let email_builder = lettre::Message::builder()
        .from("sender@example.com".parse().unwrap());
    let email = email_builder
        .to("email_2@example.com".parse().unwrap())
        .to("email_2@example.com".parse().unwrap())
        .subject("Buffalo Trace Available Product Update")
        .singlepart(
            SinglePart::builder()
                .header(lettre::message::header::ContentType::parse("text/html; charset=utf-8").unwrap())
                .body(email_body),
        )
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    // Send email
    email_transport.send(&email)?;
    println!("Email sent successfully!");
    Ok(())
}

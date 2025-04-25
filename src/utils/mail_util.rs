use crate::dto::error_dto::AppError;
use crate::dto::request_dto::RegisterRq;
use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

pub async fn send_email_activation(
    req: &RegisterRq,
    activation_key: &String,
) -> Result<String, AppError> {
    let mut to = "".to_string();
    to.push_str(&req.first_name);
    to.push_str(&" ".to_string());
    to.push_str(&req.last_name);
    to.push_str(&" <".to_string());
    to.push_str(&req.email);
    to.push_str(&">".to_string());

    let verification_link = "https://yourdomain.com/verify?token=".to_string() + &activation_key;

    let html_content = format!(
        r#"
    <html>
        <body>
            <h1>Hello!</h1>
            <p>This is a <strong>test email</strong> from Rust!</p>
            <a href="{}" target="_blank">Verify</a>
        </body>
    </html>
    "#,
        verification_link
    );

    let email = Message::builder()
        .from("NoReply <noreply@jobdomain.com>".parse().unwrap())
        .to(to.parse().unwrap())
        .subject("Activation account")
        .multipart(
            MultiPart::alternative().singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_HTML)
                    .body(html_content.to_string()),
            ),
        )
        .unwrap();
    let smtp_host: String = env::var("SMTP_HOST").unwrap_or_else(|_| "".to_string());
    let smtp_port: String = env::var("SMTP_PORT").unwrap_or_else(|_| "".to_string());
    let smtp_username: String = env::var("SMTP_USERNAME").unwrap_or_else(|_| "".to_string());
    let smtp_password: String = env::var("SMTP_PASSWORD").unwrap_or_else(|_| "".to_string());

    let creds = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::starttls_relay(smtp_host.as_str())
        .unwrap()
        .port(smtp_port.parse::<u16>().unwrap())
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => return Ok("email sent".to_string()),
        Err(e) => Err(AppError::InternalError(500000, e.to_string())),
    }
}

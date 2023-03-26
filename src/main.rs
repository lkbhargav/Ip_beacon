extern crate chrono;

use anyhow::{Ok, Result};
use chrono::offset::Utc;
use chrono::DateTime;
use clokwerk::{Interval, Scheduler};
use emitter::environment;
use emitter::ip_address::IPAddress;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::time::SystemTime;

fn main() {
    let vars = environment::EnvironmentVariables::init().expect("error initializing env vars");

    // schedule a cron job
    let mut scheduler = Scheduler::new();

    let mut ip_address = String::new();

    let mut subject = "IP address service started";

    scheduler
        .every(Interval::Minutes(vars.cron_interval))
        .run(move || {
            let gmail = vars.gmail.clone();

            let system_time = SystemTime::now();

            let datetime: DateTime<Utc> = system_time.into();

            let datetime = format!("{}", datetime.format("%m/%d/%Y %T"));

            if !ip_address.is_empty() {
                subject = "Detected IP address change";
            }

            // fetch new ip address
            let mut ip_service = IPAddress::new();

            loop {
                let ip_resp = ip_service.fetch_ip_address();

                if ip_resp.is_ok() {
                    ip_address = ip_resp.unwrap();
                    break;
                }
            }

            let res = send_email(
                &gmail.username,
                &gmail.password,
                subject,
                format!("IP address: {ip_address}\nTimestamp: {datetime} (mm/dd/yyyy)").as_str(),
            );

            if res.is_ok() {
                println!("Email sent successfully!");
            } else {
                println!("error sending email: {}", res.err().unwrap());
            }
        });

    // to keep it running every second
    loop {
        scheduler.run_pending();
    }
}

fn send_email(username: &str, password: &str, subject: &str, message: &str) -> Result<()> {
    let email = Message::builder()
        .from("DoorSensor <bhargav.lakkur@gmail.com>".parse()?)
        .reply_to("DoorSensor <bhargav.lakkur@gmail.com>".parse()?)
        .to("Bhargav Lakkur <lkbhargav9@gmail.com>".parse()?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(message))?;

    let creds = Credentials::new(username.to_string(), password.to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    mailer.send(&email)?;

    Ok(())
}

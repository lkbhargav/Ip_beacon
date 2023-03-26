extern crate chrono;

use chrono::offset::Utc;
use chrono::DateTime;
use clokwerk::{Interval, Scheduler};
use email::{Email, Relay};
use emitter::environment;
use emitter::ip_address::IPAddress;
use std::time::SystemTime;

const EMAIL_FROM: &str = "IPMonitor <bhargav.lakkur@gmail.com>";
const TO_ADDRESS: &str = "Bhargav Lakkur <lkbhargav9@gmail.com>";

fn main() {
    let vars = environment::EnvironmentVariables::init().expect("error initializing env vars");

    // schedule a cron job
    let mut scheduler = Scheduler::new();

    let mut ip_address = String::new();

    let mut subject = "IP address service started";

    let gmail = Email::new(
        EMAIL_FROM,
        EMAIL_FROM,
        &vars.gmail.username.as_str(),
        &vars.gmail.password.as_str(),
        Relay::Gmail,
    )
    .expect("error intiializing email service");

    scheduler
        .every(Interval::Minutes(vars.cron_interval))
        .run(move || {
            let system_time = SystemTime::now();

            let datetime: DateTime<Utc> = system_time.into();

            let datetime = format!("{}", datetime.format("%m/%d/%Y %T"));

            if !ip_address.is_empty() {
                subject = "Detected IP address change";
            }

            // fetch new ip address
            let mut ip_service = IPAddress::new();

            let mut just_fetched = String::new();

            // in a loop for multiple retries
            loop {
                let ip_resp = ip_service.fetch_ip_address();

                if ip_resp.is_ok() {
                    just_fetched = ip_resp.unwrap();
                    break;
                }
            }

            if ip_address.ne(&just_fetched) {
                ip_address = just_fetched;

                let res = gmail.send(
                    TO_ADDRESS,
                    subject,
                    format!("IP address: {ip_address}\nTimestamp: {datetime} (mm/dd/yyyy)")
                        .as_str(),
                );

                if res.is_err() {
                    println!("error sending email: {}", res.err().unwrap());
                }
            }
        });

    // to keep it running every second
    loop {
        scheduler.run_pending();
    }
}

use anyhow::Result;
use std::env;

#[derive(Debug, Clone)]
pub struct Gmail {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct EnvironmentVariables {
    pub gmail: Gmail,
    pub to_addresses: Vec<String>,
    // in minutes
    pub cron_interval: u32,
}

macro_rules! parse_env {
    ($key:expr, $mess:expr) => {
        env::var($key).expect($mess)
    };
}

impl EnvironmentVariables {
    pub fn init() -> Result<Self> {
        let username = parse_env!("IP_USERNAME", "IP_USERNAME is required but not found");
        let password = parse_env!(
            "IP_APP_PASSWORD",
            "IP_APP_PASSWORD is required but not found"
        );
        let cron_interval = parse_env!(
            "IP_CRON_INTERVAL",
            "IP_CRON_INTERVAL is required but not found"
        );

        let cron_interval = cron_interval.parse::<u32>()?;

        // expected format: (fname lname <fname.lname@domain.com>),(fname2 lname2 <fname2.lname2@domain.com>)
        let to_addresses = parse_env!(
            "IP_TO_ADDRESSES",
            "IP_TO_ADDRESSES is required but not found"
        );

        let to_addresses = fetch_to_addresses(&to_addresses);

        if to_addresses.len() == 0 {
            panic!("IP_TO_ADDRESSES cannot be empty");
        }

        Ok(EnvironmentVariables {
            gmail: Gmail { username, password },
            to_addresses,
            cron_interval,
        })
    }
}

fn fetch_to_addresses(to_addresses: &str) -> Vec<String> {
    to_addresses
        .split(",")
        .map(|v| v.trim())
        .map(|v| v[1..v.len() - 1].trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ideal_case() {
        let test_val =
            "(fname lname <fname.lname@domain.com>),(fname2 lname2 <fname2.lname2@domain.com>)";
        let res = fetch_to_addresses(test_val);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], "fname lname <fname.lname@domain.com>");
        assert_eq!(res[1], "fname2 lname2 <fname2.lname2@domain.com>");
    }

    #[test]
    fn test_worst_case() {
        let test_val =
            "(fname lname <fname.lname@domain.com>) , (fname2 lname2 <fname2.lname2@domain.com>)";
        let res = fetch_to_addresses(test_val);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], "fname lname <fname.lname@domain.com>");
        assert_eq!(res[1], "fname2 lname2 <fname2.lname2@domain.com>");
    }
}

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

        Ok(EnvironmentVariables {
            gmail: Gmail { username, password },
            cron_interval,
        })
    }
}

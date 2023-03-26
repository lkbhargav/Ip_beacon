use std::env;

#[derive(Debug, Clone)]
pub struct Gmail {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct EnvironmentVariables {
    pub gmail: Gmail,
}

macro_rules! parse_env {
    ($key:expr, $mess:expr) => {
        env::var($key).expect($mess)
    };
}

impl EnvironmentVariables {
    pub fn init() -> Self {
        let username = parse_env!("IP_USERNAME", "IP_USERNAME is required but not found");
        let password = parse_env!(
            "IP_APP_PASSWORD",
            "IP_APP_PASSWORD is required but not found"
        );

        EnvironmentVariables {
            gmail: Gmail { username, password },
        }
    }
}

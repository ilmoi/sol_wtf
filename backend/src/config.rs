#[derive(serde::Deserialize)]
pub struct Settings {
    pub app_port: u16,
    pub database: DbSettings,
    pub twitter: TwitterSettings,
}

#[derive(serde::Deserialize)]
pub struct DbSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub db_name: String,
}

#[derive(serde::Deserialize)]
pub struct TwitterSettings {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_token: String,
    pub access_token_secret: String,
    pub bearer_token: String,
}

impl DbSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name
        )
    }
}

// ----------------------------------------------------------------------------- fn

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();

    // todo - currently db params stored in 5 places: .env, init_db.sh, configuration, deploy_app
    settings.merge(config::File::with_name("private_config"))?;
    settings.try_into()
}

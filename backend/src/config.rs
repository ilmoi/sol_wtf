use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::convert::{TryFrom, TryInto};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub app: AppSettings,
    pub database: DbSettings,
    pub twitter: TwitterSettings,
}

#[derive(serde::Deserialize)]
pub struct AppSettings {
    pub port: u16,
    pub host: String,
    pub refresh_freq: u64,
    pub refresh_tweets_per_user: u32,
    pub followers_for_account: String,
    pub max_users: usize,
}

#[derive(serde::Deserialize)]
pub struct DbSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub db_name: String,
    pub require_ssl: bool,
}

#[derive(serde::Deserialize)]
pub struct TwitterSettings {
    // pub consumer_key: String, //v1 which I'm not using
    // pub consumer_secret: String, //v1 which I'm not using
    // pub access_token: String, //v1 which I'm not using
    // pub access_token_secret: String, //v1 which I'm not using
    pub bearer_token: String,
}

impl DbSettings {
    pub fn conn_opts(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .database(&self.db_name)
            .ssl_mode(ssl_mode)
    }
}

#[derive(PartialEq)]
pub enum Environment {
    Dev,
    Prod,
}

impl Environment {
    fn to_str(&self) -> String {
        match self {
            Environment::Dev => "dev_config".into(),
            Environment::Prod => "prod_config".into(),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match &value[..] {
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            _ => Err(format!(
                "{} is an unsupported Environment. Use either 'dev' or 'prod'.",
                value
            )),
        }
    }
}

// ----------------------------------------------------------------------------- fn

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("failed to determine current dir.");
    let config_dir = base_path.join("config");

    // 1) merge base env
    settings.merge(config::File::from(config_dir.join("base_config")).required(true))?;

    let app_env: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "dev".into())
        .try_into()
        .expect("failed to determine App Environment.");

    // 2) merge dev / prod / env
    settings.merge(config::File::from(config_dir.join(app_env.to_str())).required(true))?;

    // 3) merge secrets
    settings.merge(config::File::from(base_path.join("./secrets/twitter")).required(true))?;

    // todo - currently db params stored in all these places:
    //  [prod]
    //  config/ - proper place where db config should be recorded,
    //  [local]
    //  config/ - proper place where db config should be recorded,
    //  .env - LOCAL COMPILATION only (compilation in CICD uses sqlx-data.json),
    //  docker-compose.yml - LOCAL TESTING only,
    //  .github/workflows/deploy_app.yml - CLIPPY only,

    settings.try_into()
}

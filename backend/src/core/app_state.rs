use std::{sync::OnceLock, time::Duration};

use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::core::config::Config;

#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub database: Option<PgPool>,
}

static APP_STATE: OnceLock<AppState> = OnceLock::new();

impl AppState {
    pub async fn configure_from_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let state = match config.database_url.clone() {
            Some(database_url) => {
                let pool = PgPoolOptions::new()
                    .max_connections(config.database_max_connections)
                    .acquire_timeout(Duration::from_secs(config.database_connect_timeout_seconds))
                    .connect(&database_url)
                    .await?;

                sqlx::migrate!("./migrations").run(&pool).await?;

                Self {
                    database: Some(pool),
                }
            }
            None => Self::default(),
        };

        let _ = APP_STATE.set(state);
        Ok(())
    }

    pub fn current() -> Self {
        APP_STATE.get_or_init(Self::default).clone()
    }

    pub fn database_ready(&self) -> bool {
        self.database.is_some()
    }

    pub fn is_database_configured(&self) -> bool {
        self.database.is_some()
    }
}

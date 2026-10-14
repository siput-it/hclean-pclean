use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config = hclean_pclean_backend::core::config::Config::from_env()?;
    hclean_pclean_backend::core::app_state::AppState::configure_from_config(&config).await?;
    let listener = TcpListener::bind(config.bind_addr).await?;

    let database_ready =
        hclean_pclean_backend::core::app_state::AppState::current().database_ready();

    info!(
        address = %listener.local_addr()?,
        database_url_configured = config.database_url.is_some(),
        database_ready,
        "backend siap menerima permintaan"
    );

    axum::serve(listener, hclean_pclean_backend::http::routes::router()).await?;
    Ok(())
}

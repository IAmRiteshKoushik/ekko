mod api;

use serde::Deserialize;
use std::{error::Error, fs};
use tower_http::{catch_panic::CatchPanicLayer, trace::TraceLayer};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
struct ApiConfig {
    host: String,
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing crate. It is the recommended logging crate and does
    // the following per log-line
    // 1. on_request
    // 2. on_response
    // 3. on_eos (end of stream)
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    // Read config
    let contents = fs::read_to_string("config/api.toml")?;
    let config: ApiConfig = toml::from_str(&contents)?;

    // Construct URL
    let address = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&address).await?;

    let app = api::router::router()
        .layer(TraceLayer::new_for_http())
        .layer(CatchPanicLayer::new());

    info!("listening on http://{address}");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

// Here, we install the listeners for SIGINT/SIGTERM
async fn shutdown_signal() {
    // We register Ctrl+C as the common listener as there is underlying code
    // in tokio which handles for both Windows and Unix API
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl-C handler");
    };

    // Specific to the Unix API, we have to register SIGTERM as it is unique.
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(
            tokio::signal::unix::SignalKind::terminate(),
        )
        .expect("failed to install SIGTERM handler")
        .recv()
        .await;
    };

    /// Here's the code if we had to register SIGINT as a handler too
    ///
    /// ```
    /// let interrupt = tokio::signal::unix::signal(
    ///     tokio::signal::unix::SignalKind::interrupt(),
    /// )
    /// .expect("failed to install SIGINT handler");
    /// ```

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // This waits on both futures at once. select!() polls them together and
    // continues as soon as one completes. Here either Ctrl-C or SIGTERM
    // completes its branch so the function proceeds. We ignore the output of
    // either of the signals and proceed with {} which means there is no extra
    // branch specific-code left. This means, it prints the warning log and
    // returns.
    //
    // What this means for Axum is that the "future" is complete and it can
    // commence its graceful shutdown now. It stops accepting new connections
    // and allows in-flight requests to finish before the server future returns.
    //
    // NOTE: If the app has other background tasks, then the signal needs to be
    // passed to them separately as this does not propagate.
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    warn!("shutdown signal received")
}

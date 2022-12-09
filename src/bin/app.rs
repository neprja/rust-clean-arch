use rust::{config::Config, controller::http::server, tracer, usecase};

use anyhow::{Context, Result};
use opentelemetry::global;
use rust::infrastructure::{microservice_name, repository};
use tokio::signal;
use tracing::{info, level_filters::LevelFilter};

#[tokio::main]
async fn main() -> Result<()> {
    let c = Config::new().context("config")?;
    tracer::tracer(LevelFilter::INFO).context("tracer")?;

    let usecase = usecase::UseCase::new(
        repository::Repository::new(),
        microservice_name::MicroserviceName::new().context("new microservice_name")?,
    );

    server(c.port, wait_signal(), usecase)
        .await
        .context("server")?;

    Ok(())
}

async fn wait_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    info!("App started!");

    tokio::select! {
        _ = ctrl_c => {info!("SIGINT signal received")},
        _ = terminate => {info!("SIGTERM signal received")},
    }

    global::shutdown_tracer_provider();
}

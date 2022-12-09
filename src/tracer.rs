use anyhow::{Context, Result};
use opentelemetry::global;
use opentelemetry::sdk::{
    propagation::TraceContextPropagator,
    trace::{self, Sampler},
    Resource,
};
use opentelemetry_otlp::WithExportConfig;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

pub fn tracer(level: LevelFilter) -> Result<()> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
        .with_trace_config(
            trace::config()
                .with_resource(Resource::default())
                .with_sampler(Sampler::AlwaysOn),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .context("tracer install")?;

    let otel = tracing_opentelemetry::layer()
        .with_tracer(tracer)
        .with_filter(level);

    let stdout = tracing_subscriber::fmt::layer().with_filter(level);

    tracing_subscriber::registry()
        .with(otel)
        .with(stdout)
        .try_init()
        .context("registry try_init")?;

    Ok(())
}

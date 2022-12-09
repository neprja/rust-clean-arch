use crate::infrastructure::microservice_name::MicroserviceName;
use anyhow::{Context, Result};
use opentelemetry::global;
use opentelemetry_http::HeaderInjector;
use reqwest::{Method, Request, RequestBuilder};
use tracing::{info, Span};
use tracing_attributes::instrument;
use tracing_opentelemetry::OpenTelemetrySpanExt;

impl MicroserviceName {
    #[instrument(skip(self), fields(otel.kind="client"))]
    pub async fn get_profile(&self, id: String) -> Result<String> {
        let mut req = self
            .client
            .get("http://127.0.0.1:8081/v1/profile/req")
            .build()
            .context("build")?;

        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(
                &Span::current().context(),
                &mut HeaderInjector(req.headers_mut()),
            )
        });

        let resp = self.client.execute(req).await.context("execute")?;

        info!(answer = resp.status().as_str());

        Ok("ok: ".to_string() + &*resp.status().to_string())
    }
}

use crate::usecase::UseCase;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Html;
use tracing::{info, Span};

pub(crate) async fn get_profile(
    State(usecase): State<UseCase>,
    Path(id): Path<String>,
) -> Html<String> {
    let msg = usecase.get_profile(id).await.unwrap();

    Span::current().record("otel.status_code", &"ok");
    Span::current().record("http.status_code", i32::from(StatusCode::OK.as_u16()));

    info!(msg);

    Html(msg)
}

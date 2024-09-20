use axum::{
    response::sse::{Event, Sse},
    routing::get,
    Router,
};
use futures::stream::{self, Stream};
use std::{convert::Infallible, net::SocketAddr, time::Duration};
use tokio_stream::StreamExt as _;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
        }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    let addr = SocketAddr::from(([0,0,0,0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let app = Router::new()
        .route("/", get(get_root));

    tracing::debug!("listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}


async fn get_root(
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    tracing::debug!("root gotten!");
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
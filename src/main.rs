use dotenvy::{dotenv, var};
use o_ads_backend::{AppState, router, subscriber, trace_layer};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    subscriber();
    tracing::debug!("starting up");
    dotenv().expect("dotenv");
    let conn_str = var("DATABASE_URL").expect("missing DATABASE_URL");
    let state = AppState::new(&conn_str)
        .await
        .expect("error initializing appstate...is the db running?");
    let app = router()
        .layer(trace_layer())
        .with_state(state)
        .into_make_service_with_connect_info::<SocketAddr>();
    let addr = format!(
        "0.0.0.0:{}",
        var("BACKEND_PORT").unwrap_or(String::from("8080"))
    );
    let listener = TcpListener::bind(addr)
        .await
        .expect("error creating listener on {addr}");
    axum::serve(listener, app)
        .await
        .expect("error starting app");
}

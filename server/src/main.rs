use axum::{response::IntoResponse, routing::get, Router};
use clap::Parser;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use tower::ServiceBuilder;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Opt {
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,
    #[clap(short = 'a', long = "addr", default_value = "localhost")]
    addr: String,
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,
    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!("{}, hyper=info,mio=info", opt.log_level),
        )
    }
    tracing_subscriber::fmt::init();

    let service = ServeDir::new("assets").fallback(ServeFile::new("assets/not_found.html"));

    let app = Router::new()
        .route("/api/hello", get(hello))
        .fallback_service(service)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    log::info!("listening on http://{}", sock_addr);

    let listener = tokio::net::TcpListener::bind(&sock_addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> impl IntoResponse {
    "Hello from server!"
}

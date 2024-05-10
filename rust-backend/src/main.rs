#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use std::time::Duration;

use axum::{
    body::Body,
    Extension,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    Router, routing::{get, post},
};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::sync::{mpsc, mpsc::Sender};

use handler::{create_service, get_services};
use repository::db::postgres::flags as flags_repo;
use repository::db::postgres::services as services_repo;
use repository::db::postgres::streams as streams_repo;
use sniffer::Sniffer;

pub mod domain;
pub mod handler;
pub mod repository;
pub mod sniffer;

#[derive(Clone)]
pub struct AppContext {
    pub db: PgPool,
    pub tx: Sender<u16>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let (tx, rx) = mpsc::channel(1);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect("postgresql://localhost/bezdna?user=user&password=1234&port=5433")
        .await
        .unwrap();

    let _flags_repo = flags_repo::Repository::new(pool.clone());
    let _services_repo = services_repo::Repository::new(pool.clone());
    let _stream_repo = streams_repo::Repository::new(pool.clone());

    let app = Router::new()
        .route("/get-services", get(get_services))
        .route("/create-service", post(create_service))
        .layer(middleware::from_fn(info_middleware))
        .layer(Extension(AppContext {
            db: pool.clone(),
            tx,
        }));

    futures_util::future::join_all(vec![
        tokio::spawn(async move {
            Sniffer::new(
                pool,
                "lo0",
                chrono::Duration::seconds(10),
                chrono::Duration::seconds(20),
            )
            .run(rx)
            .await
            .expect("sniffer")
        }),
        tokio::spawn(async move {
            axum::Server::bind(&"0.0.0.0:3123".parse().unwrap())
                .serve(app.into_make_service())
                .await
                .expect("server")
        }),
    ])
    .await;
}

async fn info_middleware(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::info!("{:?}", req.uri());

    Ok(next.run(req).await)
}

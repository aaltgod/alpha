#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Router,
};
use sqlx::postgres::PgPoolOptions;

use handler::{
    create_rule::create_rule, delete_rule::delete_rule, delete_service::delete_service,
    delete_service_to_rules::delete_service_to_rules, get_rules::get_rules,
    get_services::get_services, get_streams_by_service_ids::get_streams_by_service_ids,
    update_rule::update_rule, upsert_service::upsert_service,
};
use repository::db::postgres::packets as packets_repo;
use repository::db::postgres::services as services_repo;
use repository::db::postgres::streams as streams_repo;
use sniffer::Sniffer;
use tower_http::cors::CorsLayer;

use crate::handler::types::AppContext;
use crate::sniffer::external_types::PORTS_TO_SNIFF;

mod config;
mod domain;
mod handler;
mod repository;
mod sniffer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let postgres_config =
        config::provide_postgres_config().expect("couldn't provide postgres config");
    let sniffer_config = config::provide_sniffer_config().expect("couldn't provide sniffer config");
    let server_config = config::provide_server_config().expect("couldn't server sniffer config");

    let pool = PgPoolOptions::new()
        .max_connections(postgres_config.max_connections)
        .acquire_timeout(postgres_config.timeout)
        .connect(&postgres_config.database_url)
        .await
        .expect("couldn't init postgres pool");

    let streams_repo = streams_repo::Repository::new(pool.clone());
    let packets_repo = packets_repo::Repository::new(pool.clone());
    let services_repo = services_repo::Repository::new(pool.clone());

    {
        let mut ports_to_sniff = PORTS_TO_SNIFF.lock().await;
        services_repo
            .get_all_services()
            .await
            .expect("couldn't get services from db")
            .into_iter()
            .for_each(|s| {
                ports_to_sniff.insert(s.port, ());
            });
    }

    let app = Router::new()
        .route("/get-services", get(get_services))
        .route("/upsert-service", post(upsert_service))
        .route("/delete-service", delete(delete_service))
        .route(
            "/get-streams-by-service-ids",
            get(get_streams_by_service_ids),
        )
        .route("/get-rules", get(get_rules))
        .route("/create-rule", post(create_rule))
        .route("/update-rule", put(update_rule))
        .route("/delete-rule", delete(delete_rule))
        .route("/delete-service-to-rules", delete(delete_service_to_rules))
        .layer(CorsLayer::permissive())
        .layer(middleware::from_fn(info_middleware))
        .layer(Extension(AppContext {
            services_repo: services_repo.clone(),
            streams_repo: streams_repo.clone(),
        }));

    futures_util::future::join_all(vec![
        tokio::spawn(async move {
            Sniffer::new(
                streams_repo,
                packets_repo,
                sniffer_config.interface,
                sniffer_config.tcp_stream_ttl,
                sniffer_config.max_stream_ttl,
            )
            .run()
            .await
            .expect("run sniffer")
        }),
        tokio::spawn(async move {
            axum::Server::bind(
                &format!("{}:{}", server_config.host, server_config.port)
                    .parse()
                    .expect("invalid server addr"),
            )
            .serve(app.into_make_service())
            .await
            .expect("run server")
        }),
    ])
    .await;
}

async fn info_middleware(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::info!("{:?} {:?}", req.uri(), req.body());

    Ok(next.run(req).await)
}

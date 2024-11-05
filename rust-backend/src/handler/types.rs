use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::domain;
use crate::repository::db::postgres::{services as services_repo, streams as streams_repo};

#[derive(Clone)]
pub struct AppContext {
    pub services_repo: services_repo::Repository,
    pub streams_repo: streams_repo::Repository,
}

pub enum AppResponse {
    OK,
    CREATED,
}

impl IntoResponse for AppResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK => StatusCode::OK.into_response(),
            Self::CREATED => StatusCode::CREATED.into_response(),
        }
    }
}

pub enum AppError {
    InternalServerError(anyhow::Error),
    BadRequest(String, anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            Self::InternalServerError(e) => {
                error!("{e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Внутренняя ошибка".to_owned(),
                )
            }
            Self::BadRequest(msg, e) => {
                error!("{e}");
                (StatusCode::BAD_REQUEST, msg)
            }
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub name: String,
    pub port: i16,
}

impl From<domain::Service> for Service {
    fn from(service: domain::Service) -> Self {
        Service {
            id: service.id,
            name: service.name,
            port: service.port,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Services {
    pub services: Vec<Service>,
}

impl From<Vec<domain::Service>> for Services {
    fn from(services: Vec<domain::Service>) -> Self {
        Services {
            services: services
                .into_iter()
                .map(|s| Service {
                    id: s.id,
                    name: s.name,
                    port: s.port,
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Packet {
    pub payloads: Vec<Payload>,
    pub direction: String,
    pub at: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Payload {
    pub text: String,
    pub rule_regexp: String,
    pub rule_color: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Stream {
    pub id: i64,
    pub service_name: String,
    pub service_port: i16,
    pub rule_regexps: Vec<String>,
    pub started_at: String,
    pub ended_at: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct StreamWithPackets {
    pub stream: Stream,
    pub packets: Vec<Packet>,
}

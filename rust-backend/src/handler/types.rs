use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
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
                (StatusCode::INTERNAL_SERVER_ERROR, "Внутренняя ошибка".to_owned())
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
    pub flag_regexp: String,
}

impl From<domain::Service> for Service {
    fn from(service: domain::Service) -> Self {
        Service {
            id: service.id,
            name: service.name,
            port: service.port,
            flag_regexp: service.flag_regexp.to_string(),
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
                    flag_regexp: s.flag_regexp.to_string(),
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Packet {
    pub direction: String,
    pub payload: String,
    pub at: String,
    pub flag_regexp: String,
    pub color: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct StreamIDWithPackets {
    pub stream_id: i64,
    pub packets: Vec<Packet>,
}
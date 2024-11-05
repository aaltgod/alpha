use std::collections::HashMap;

use crate::domain;
use crate::handler::types;
use crate::handler::types::{AppError, Packet, StreamWithPackets};
use axum::{Extension, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::types::Stream;

pub async fn get_streams_by_service_ids(
    ctx: Extension<types::AppContext>,
    Json(req): Json<GetStreamsByServiceIDsRequest>,
) -> Result<Json<GetStreamsByServiceIDsResponse>, AppError> {
    let rules_by_service = ctx
        .services_repo
        .get_rules_by_service(req.service_ids)
        .await
        .map_err(AppError::InternalServerError)?;

    let services = rules_by_service.keys();

    let packets_by_stream = ctx
        .streams_repo
        .get_packets_by_stream(
            services.into_iter().map(|s| s.port).collect(),
            req.last_stream_id,
            // TODO: сделать limit настраиваемым.
            20,
        )
        .await
        .map_err(AppError::InternalServerError)?;

    let mut rules_by_service_id: HashMap<i64, Vec<domain::Rule>> =
        HashMap::with_capacity(rules_by_service.len());

    for (service, rules) in rules_by_service {
        rules_by_service_id.insert(service.id, rules);
    }

    let mut resp = GetStreamsByServiceIDsResponse {
        stream_with_packets: Vec::with_capacity(packets_by_stream.len()),
    };

    for (stream, packets) in packets_by_stream {
        let mut regexps: HashMap<String, ()> = HashMap::new();
        let mut started_at: DateTime<Utc> = Default::default();
        let mut ended_at: DateTime<Utc> = Default::default();

        packets.into_iter().for_each(|p| {
            rules_by_service_id
                .get(&stream.id)
                .unwrap()
                .into_iter()
                .for_each(|r| {
                    if r.regexp.captures(p.payload.as_bytes()).is_some() {
                        regexps.entry(r.regexp.to_string());
                    }

                    if p.at < started_at {
                        started_at = p.at
                    }

                    if p.at > ended_at {
                        ended_at = p.at
                    }
                });
        });

        // resp.stream_with_packets.push(StreamWithPackets {
        //     stream: Stream {
        //         id: stream.id,
        //         service_name: ,
        //         service_port: 10,
        //         rule_regexps: regexps.keys().cloned().collect(),
        //         started_at: started_at.to_string(),
        //         ended_at: ended_at.to_string(),
        //     },
        //     packets: packets
        //         .into_iter()
        //         .map(|packet| Packet {
        //             direction: packet.direction.to_string(),
        //             payload: packet.payload,
        //             at: packet.at.to_string(),
        //             flag_regexp: services
        //                 .iter()
        //                 .find(|service| service.port.eq(&stream.service_port))
        //                 .map_or("".to_string(), |s| s.flag_regexp.to_string()),
        //             color: match packet.direction {
        //                 domain::PacketDirection::IN => "#33FF46".to_string(),
        //                 domain::PacketDirection::OUT => "#FF3333".to_string(),
        //             },
        //         })
        //         .collect(),
        // })
    }

    Ok(Json(resp))
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetStreamsByServiceIDsRequest {
    pub service_ids: Vec<i64>,
    pub last_stream_id: i64,
}

#[derive(Clone, Debug, Serialize)]
pub struct GetStreamsByServiceIDsResponse {
    pub stream_with_packets: Vec<StreamWithPackets>,
}

use anyhow::anyhow;
use sqlx::PgPool;
use std::collections::BTreeMap;

use crate::domain;
use crate::repository::db::postgres::packets::types;

#[derive(Clone)]
pub struct Repository {
    db: PgPool,
}

impl Repository {
    pub fn new(db: PgPool) -> Self {
        Repository { db }
    }

    pub async fn create_streams(
        &self,
        streams: Vec<domain::Stream>,
    ) -> Result<Vec<i64>, anyhow::Error> {
        let qty = streams.len();
        let mut service_ports: Vec<i32> = Vec::with_capacity(qty);

        streams.into_iter().for_each(|stream| {
            service_ports.push(stream.service_port as u32 as i32);
        });

        let records = sqlx::query!(
            r#"
        INSERT INTO streams
		    (
			    service_port
		    )
		SELECT
		      UNNEST($1::integer[]) AS service_port
        RETURNING id
        "#,
            service_ports.as_slice(),
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| anyhow!(e.to_string()))?;

        Ok(records.into_iter().map(|record| record.id).collect())
    }

    pub async fn get_packets_by_stream(
        &self,
        ports: Vec<i32>,
        stream_id: i64,
        limit: i64,
    ) -> Result<BTreeMap<domain::Stream, Vec<domain::Packet>>, anyhow::Error> {
        let records = match sqlx::query!(
            r#"
        SELECT
            streams.id AS stream_id,
            streams.service_port AS service_port,
            packets.direction AS "packet_direction: types::PacketDirection",
            packets.payload,
            packets.at FROM (
                        SELECT id, service_port FROM streams
                        WHERE
                            service_port = ANY($1::INTEGER[])
                        AND
                            id > $2
                        LIMIT $3
                    ) AS streams
                        INNER JOIN packets ON streams.id = packets.stream_id
        ORDER BY streams.id, packets.at
        "#,
            ports as _,
            stream_id,
            limit
        )
        .fetch_all(&self.db)
        .await
        {
            Ok(res) => res,
            Err(e) => {
                return match e {
                    sqlx::Error::RowNotFound => Ok(BTreeMap::default()),
                    _ => Err(anyhow!(e.to_string())),
                };
            }
        };

        let mut result: BTreeMap<domain::Stream, Vec<domain::Packet>> = BTreeMap::new();

        records.into_iter().for_each(|record| {
            let packet = domain::Packet {
                id: 0,
                direction: record.packet_direction.into(),
                payload: record.payload,
                stream_id: record.stream_id,
                at: record.at,
            };

            result
                .entry(domain::Stream {
                    id: record.stream_id,
                    service_port: record.service_port as i32,
                })
                .and_modify(|packets| packets.push(packet.clone()))
                .or_insert(vec![packet]);
        });

        Ok(result)
    }

    pub async fn get_last_streams(
        &self,
        limit: i64,
    ) -> Result<BTreeMap<domain::Stream, Vec<domain::Packet>>, anyhow::Error> {
        let records = match sqlx::query!(
            r#"
        SELECT
            streams.id AS stream_id,
            streams.service_port AS service_port,
            packets.direction AS "packet_direction: types::PacketDirection",
            packets.payload,
            packets.at FROM (
                        SELECT id, service_port FROM streams
                        ORDER BY id DESC
                        LIMIT $1
                    ) AS streams
                        INNER JOIN packets ON streams.id = packets.stream_id
        ORDER BY streams.id, packets.at
        "#,
            limit
        )
        .fetch_all(&self.db)
        .await
        {
            Ok(res) => res,
            Err(e) => {
                return match e {
                    sqlx::Error::RowNotFound => Ok(BTreeMap::default()),
                    _ => Err(anyhow!(e.to_string())),
                };
            }
        };

        let mut result: BTreeMap<domain::Stream, Vec<domain::Packet>> = BTreeMap::new();

        records.into_iter().for_each(|record| {
            let packet = domain::Packet {
                id: 0,
                direction: record.packet_direction.into(),
                payload: record.payload,
                stream_id: record.stream_id,
                at: record.at,
            };

            result
                .entry(domain::Stream {
                    id: record.stream_id,
                    service_port: record.service_port as i32,
                })
                .and_modify(|packets| packets.push(packet.clone()))
                .or_insert(vec![packet]);
        });

        Ok(result)
    }
}

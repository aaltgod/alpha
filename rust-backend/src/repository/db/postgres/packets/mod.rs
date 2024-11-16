use anyhow::anyhow;
use sqlx::PgPool;

use crate::domain;

pub mod types;

pub struct Repository {
    db: PgPool,
}

impl Repository {
    pub fn new(db: PgPool) -> Self {
        Repository { db }
    }

    pub async fn insert_packets(&self, packets: Vec<domain::Packet>) -> Result<(), anyhow::Error> {
        let qty = packets.len();
        let mut directions: Vec<types::PacketDirection> = Vec::with_capacity(qty);
        let mut payloads: Vec<String> = Vec::with_capacity(qty);
        let mut stream_ids: Vec<i64> = Vec::with_capacity(qty);
        let mut at: Vec<chrono::NaiveDateTime> = Vec::with_capacity(qty);

        packets.into_iter().for_each(|packet| {
            directions.push(types::PacketDirection::from(packet.direction));
            payloads.push(packet.payload);
            stream_ids.push(packet.stream_id as i64);
            at.push(packet.at.naive_utc());
        });

        sqlx::query!(
            r#"
            INSERT INTO packets
                (
                    direction,
                    payload,
                    stream_id,
                    at
                )
            SELECT
                UNNEST($1::packet_direction[]) AS direction,
                UNNEST($2::text[]) AS payload,
                UNNEST($3::bigint[]) AS stream_id,
                UNNEST($4::timestamp[]) AS at
        "#,
            directions as _,
            payloads.as_slice(),
            stream_ids.as_slice(),
            at.as_slice(),
        )
        .execute(&self.db)
        .await
        .map_err(|e| anyhow!(e.to_string()))?;

        Ok(())
    }

    pub async fn get_packets_by_stream_ids(
        &self,
        stream_ids: Vec<u64>,
    ) -> Result<Vec<domain::Packet>, anyhow::Error> {
        let stream_ids: Vec<i64> = stream_ids.into_iter().map(|id| id as i64).collect();

        let records = match sqlx::query!(
            r#"
        SELECT
            id,
            direction AS "packet_direction: types::PacketDirection",
            payload,
            stream_id,
            at
        FROM packets
        WHERE stream_id = ANY($1::bigint[])
        "#,
            stream_ids.as_slice()
        )
        .fetch_all(&self.db)
        .await
        {
            Ok(res) => res,
            Err(e) => {
                return match e {
                    sqlx::Error::RowNotFound => Ok(vec![]),
                    _ => Err(anyhow!(e.to_string())),
                };
            }
        };

        let mut packets: Vec<domain::Packet> = Vec::with_capacity(records.len());

        for record in records.into_iter() {
            packets.push(domain::Packet {
                id: record.id,
                direction: record.packet_direction.into(),
                payload: record.payload,
                stream_id: record.stream_id,
                at: record.at,
            })
        }

        Ok(packets)
    }
}

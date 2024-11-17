use std::collections::HashMap;
use std::ops::Sub;
use std::sync::Arc;
use std::time;

use anyhow::anyhow;
use pnet::datalink::{channel, Config};
use pnet::packet::ethernet::EtherTypes::Ipv4;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::tcp::{TcpFlags, TcpPacket};
use pnet::packet::PrimitiveValues;
use pnet::{
    datalink::Channel::Ethernet,
    packet::{ethernet::EthernetPacket, ipv4::Ipv4Packet, Packet},
};
use tokio::sync::Mutex;

use crate::domain;
use crate::repository::db::postgres::packets as packets_repo;
use crate::repository::db::postgres::streams as streams_repo;
use crate::sniffer::external_types::PORTS_TO_SNIFF;
use crate::sniffer::internal_types::{PortPair, TcpPacketInfo};

pub mod external_types;
mod internal_types;

pub struct Sniffer {
    streams_repo: streams_repo::Repository,
    packets_repo: packets_repo::Repository,
    interface_name: String,
    tcp_packet_info_by_port_pair: Arc<Mutex<HashMap<PortPair, Vec<TcpPacketInfo>>>>,
    tcp_stream_ttl: chrono::Duration,
    max_stream_ttl: chrono::Duration,
}

impl Sniffer {
    pub fn new(
        streams_repo: streams_repo::Repository,
        packets_repo: packets_repo::Repository,
        interface_name: String,
        tcp_stream_ttl: chrono::Duration,
        max_stream_ttl: chrono::Duration,
    ) -> Self {
        Sniffer {
            streams_repo,
            packets_repo,
            interface_name,
            tcp_packet_info_by_port_pair: Arc::new(Mutex::new(HashMap::new())),
            tcp_stream_ttl,
            max_stream_ttl,
        }
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        let arc_self = Arc::new(self);
        let self_handler = Arc::clone(&arc_self);
        let self_manager = Arc::clone(&arc_self);

        let interface = pnet::datalink::interfaces()
            .into_iter()
            .find(|interface| interface.name.eq(&self_handler.interface_name));

        if interface.is_none() {
            return Err(anyhow!(
                "interface `{}` doesn't exist",
                &self_handler.interface_name
            ));
        }

        let (_tx, mut rx) = match channel(&interface.unwrap(), Config::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => return Err(anyhow!("INVALID type")),
            Err(e) => return Err(anyhow!(e.to_string())),
        };

        futures_util::future::join_all(vec![
            tokio::spawn(async move {
                loop {
                    match rx.next() {
                        Ok(packet) => {
                            self_handler
                                .handle_eth_packet(EthernetPacket::new(packet).expect("OK"))
                                .await
                        }
                        Err(e) => error!("{}", e),
                    }
                }
            }),
            tokio::spawn(async move { self_manager.manage_tcp_streams().await }),
        ])
        .await;

        Ok(())
    }

    /// Обрабатывает TCP стримы, которые были сохранены во временный буфер
    /// в процессе прослушивания трафика.
    async fn manage_tcp_streams(&self) {
        loop {
            tokio::time::sleep(time::Duration::from_secs(5)).await;

            let mut tcp_packet_info_by_port_pair = self.tcp_packet_info_by_port_pair.lock().await;

            // Пары портов, стримы которых нужно сохранить в бд.
            let mut completed_stream_port_pairs: Vec<PortPair> =
                Vec::with_capacity(tcp_packet_info_by_port_pair.len());

            for (port_pair, tcp_packet_infos) in tcp_packet_info_by_port_pair.iter() {
                if !tcp_packet_infos.is_empty() {
                    let time_now = chrono::Local::now().to_utc();

                    // Считаем, что стрим нужно сохранить при срабатывании одного из следующих
                    // условий:
                    //
                    // последний пакет имеет признак о том, что клиент закрыл соединение;
                    if tcp_packet_infos.last().unwrap().completed
                        // клиент не присылал пакеты в течение времени, указанного в tcp_stream_ttl;
                        || time_now
                        .sub(tcp_packet_infos.last().unwrap().at)
                        .gt(&self.tcp_stream_ttl)
                        // с момента отправки от клиента первого пакета прошло времени больше,
                        // чем указано в max_stream_ttl. Это условие служит для избежания хранения
                        // в памяти стрима, пакеты которого длительное время продолжают отправляться.
                        || time_now
                        .sub(tcp_packet_infos.first().unwrap().at)
                        .gt(&self.max_stream_ttl)
                    {
                        info!("DONE: {} {:?}", tcp_packet_infos.len(), port_pair);

                        completed_stream_port_pairs.push(*port_pair);
                    }
                }
            }

            info!(
                "completed_stream_port_pairs LEN: {}",
                completed_stream_port_pairs.len()
            );

            let mut streams_to_create: Vec<domain::Stream> =
                Vec::with_capacity(completed_stream_port_pairs.len());

            for pair in completed_stream_port_pairs.iter() {
                streams_to_create.push(domain::Stream {
                    id: 0,
                    service_port: pair.dst,
                });
            }

            if !streams_to_create.is_empty() {
                // Сначала создаем стримы, потом сохраняем к ним пакеты.
                let stream_ids = self
                    .streams_repo
                    .create_streams(streams_to_create)
                    .await
                    .unwrap();

                let mut packets_to_create: Vec<domain::Packet> = Vec::new();

                for (idx, pair) in completed_stream_port_pairs.iter().enumerate() {
                    let tcp_packet_info = tcp_packet_info_by_port_pair.get(pair).unwrap();

                    for info in tcp_packet_info {
                        // Не хотим хранить в бд пустые пакеты.
                        if !info.payload.is_empty() {
                            packets_to_create.push(domain::Packet {
                                id: 0,
                                direction: info.packet_direction,
                                // FIXME: cringe moment
                                payload: info.payload.as_str().to_string(),
                                // пакет по текущему idx относится к стриму по этому же idx из stream_ids.
                                stream_id: *stream_ids.get(idx).unwrap(),
                                at: info.at,
                            })
                        }
                    }
                }

                self.packets_repo
                    .insert_packets(packets_to_create)
                    .await
                    .unwrap();

                completed_stream_port_pairs.into_iter().for_each(|pair| {
                    // Очищаем память.
                    tcp_packet_info_by_port_pair.remove(&pair);
                });
            }
        }
    }

    async fn handle_eth_packet(&self, packet: EthernetPacket<'_>) {
        if packet.get_ethertype() == Ipv4
            // TODO: почему-то на loopback'е не возвращает тип протокола.
            // Поэтому обрабатываем этот момент так.
            || packet.get_ethertype().to_primitive_values().0 == 0
        {
            let ipv4_packet = Ipv4Packet::new(packet.payload()).unwrap();

            if ipv4_packet.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {
                let tcp_packet = TcpPacket::new(ipv4_packet.payload()).unwrap();

                self.handle_tcp_packet(tcp_packet).await;
            }
        }
    }

    async fn handle_tcp_packet(&self, packet: TcpPacket<'_>) {
        let source_port = packet.get_source() as i32;
        let destination_port = packet.get_destination() as i32;

        let (port_pair, packet_direction): (PortPair, Option<domain::PacketDirection>) = {
            let ports = PORTS_TO_SNIFF.lock().await;

            if ports.contains_key(&destination_port) {
                (
                    PortPair {
                        src: source_port,
                        dst: destination_port,
                    },
                    Some(domain::PacketDirection::IN),
                )
            } else if ports.contains_key(&source_port) {
                (
                    PortPair {
                        src: destination_port,
                        dst: source_port,
                    },
                    Some(domain::PacketDirection::OUT),
                )
            } else {
                return;
            }
        };

        if packet_direction.is_none() {
            return;
        }

        let mut tcp_packet_info_by_port_pair = self.tcp_packet_info_by_port_pair.lock().await;

        let time_now = chrono::Local::now().to_utc();

        let payload = packet.payload();
        let mut info = TcpPacketInfo {
            payload: String::from_utf8(payload.to_vec()).unwrap(),
            packet_direction: packet_direction.unwrap(),
            completed: false,
            at: time_now,
        };

        tcp_packet_info_by_port_pair
            .entry(port_pair)
            .and_modify(|i| i.push(info.clone()))
            .or_insert(vec![info.clone()]);

        info!(
            "tcp_packet_info_by_port_pair LEN: {}",
            tcp_packet_info_by_port_pair.len()
        );

        if packet.get_flags().eq(&TcpFlags::FIN) || packet.get_flags().eq(&TcpFlags::RST) {
            tcp_packet_info_by_port_pair
                .entry(port_pair)
                .and_modify(|i| {
                    info.completed = true;

                    i.push(info)
                });
        };
    }
}

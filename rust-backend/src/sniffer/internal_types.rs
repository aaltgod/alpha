use crate::domain;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct PortPair {
    pub src: i32,
    pub dst: i32,
}

#[derive(Debug, Clone)]
pub struct TcpPacketInfo {
    pub payload: String,
    pub packet_direction: domain::PacketDirection,
    pub completed: bool,
    pub at: chrono::DateTime<chrono::Utc>,
}

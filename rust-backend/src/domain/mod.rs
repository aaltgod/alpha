use std::cmp::Ordering;
use std::fmt::Display;

use regex::bytes;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Service {
    pub id: i64,
    pub name: String,
    pub port: i32,
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub id: i64,
    pub name: String,
    pub packet_direction: PacketDirection,
    pub regexp: bytes::Regex,
    pub color: String,
}

#[derive(Clone, Debug)]
pub struct ServiceWithRules {
    pub service: Service,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Eq)]
pub struct Stream {
    pub id: i64,
    pub service_port: i32,
}

impl Ord for Stream {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Stream {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Stream {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Clone)]
pub struct Packet {
    pub id: i64,
    pub direction: PacketDirection,
    pub payload: String,
    pub stream_id: i64,
    pub at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PacketDirection {
    IN,
    OUT,
}

impl PacketDirection {
    pub fn from_str(s: &str) -> Option<PacketDirection> {
        match s {
            "IN" => Some(PacketDirection::IN),
            "OUT" => Some(PacketDirection::OUT),
            _ => None,
        }
    }
}

impl Display for PacketDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PacketDirection::IN => "IN".to_string(),
            PacketDirection::OUT => "OUT".to_string(),
        };
        write!(f, "{}", str)
    }
}

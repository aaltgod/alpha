use sqlx::postgres::{PgHasArrayType, PgTypeInfo};

use crate::domain;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "packet_direction")]
pub enum PacketDirection {
    IN,
    OUT,
}

impl From<domain::PacketDirection> for PacketDirection {
    fn from(value: domain::PacketDirection) -> Self {
        match value {
            domain::PacketDirection::IN => PacketDirection::IN,
            domain::PacketDirection::OUT => PacketDirection::OUT,
        }
    }
}

impl Into<domain::PacketDirection> for PacketDirection {
    fn into(self) -> domain::PacketDirection {
        match self {
            PacketDirection::IN => domain::PacketDirection::IN,
            PacketDirection::OUT => domain::PacketDirection::OUT,
        }
    }
}

impl PgHasArrayType for PacketDirection {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_packet_direction")
    }
}

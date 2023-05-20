use std::{net::Ipv4Addr, time::Duration};

use super::request_param::Area;
use serde::{Deserialize, Serialize};

/// Client Connection Type
/// 16 possible connections limited by the
/// hardware The types are defined from the
/// highest to lowest priority
/// The basic connections are the first which
/// would be closed if there aren't enough
/// resources
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize,
)]
pub enum ConnectionType {
    /// Connect to the PLC programming console
    /// (Programmiergeräte). German for
    /// programming device.
    PG    = 1,
    /// Connect to the PLC Siemens HMI panel
    OP    = 2,
    /// Basic connection for generic data
    /// transfer connection
    /// 14 Basic connections
    Basic = 3
}

impl Default for ConnectionType {
    fn default() -> Self {
        Self::OP
    }
}

#[derive(
    Debug, Clone, Serialize, Deserialize,
)]
pub enum ConnectMode {
    Tsap {
        conn_type:   ConnectionType,
        local_tsap:  u16,
        remote_tsap: u16
    },
    RackSlot {
        conn_type: ConnectionType,
        rack:      u16,
        slot:      u16
    }
}
impl ConnectMode {
    pub fn init_tsap(
        conn_type: ConnectionType,
        local_tsap: u16,
        remote_tsap: u16
    ) -> Self {
        Self::Tsap {
            conn_type,
            local_tsap,
            remote_tsap
        }
    }

    pub fn init_rack_slot(
        conn_type: ConnectionType,
        rack: u16,
        slot: u16
    ) -> Self {
        Self::RackSlot {
            conn_type,
            rack,
            slot
        }
    }

    pub fn conn_type(&self) -> &ConnectionType {
        match self {
            ConnectMode::Tsap {
                conn_type,
                ..
            } => conn_type,
            ConnectMode::RackSlot {
                conn_type,
                ..
            } => conn_type
        }
    }

    pub fn local_tsap(&self) -> [u8; 2] {
        match self {
            ConnectMode::Tsap {
                local_tsap,
                ..
            } => [
                (local_tsap >> 8) as u8,
                *local_tsap as u8
            ],
            ConnectMode::RackSlot { .. } => {
                [0x01, 0x00]
            },
        }
    }

    pub fn remote_tsap(&self) -> [u8; 2] {
        let remote_tsap = match self {
            ConnectMode::Tsap {
                remote_tsap,
                ..
            } => *remote_tsap,
            ConnectMode::RackSlot {
                rack,
                slot,
                conn_type
            } => {
                ((*conn_type as u16) << 8)
                    + (rack * 0x20)
                    + slot
            },
        };
        [
            (remote_tsap >> 8) as u8,
            remote_tsap as u8
        ]
    }
}

use s7_comm::{ItemRequest, TransportSize};
use serde::{Deserialize, Serialize};

use crate::Error;
use std::ops::Deref;

type S7Area = s7_comm::Area;
// Area ID
#[derive(
    Debug, Copy, Clone, Serialize, Deserialize,
)]
#[allow(dead_code)]
pub enum Area {
    ProcessInput(DataSizeType),
    ProcessOutput(DataSizeType),
    /// Merkers are address registers within the
    /// CPU. The number of available flag
    /// bytes depends on the respective CPU and
    /// can be taken from the technical data.
    /// You can use flag bits, flag bytes, flag
    /// words or flag double words in a PLC
    /// program.
    Merker(DataSizeType),
    /// German thing, means building blocks
    /// This is your storage  : db number,
    /// DataSizeType
    DataBausteine(u16, DataSizeType),
    V(DataSizeType), /* Counter,
                      * Timer, */
}

impl Into<ItemRequest> for Area {
    fn into(self) -> ItemRequest {
        match &self {
            Area::ProcessInput(ds) => {
                ItemRequest::new(
                    ds.to_transport_size(),
                    s7_comm::DbNumber::NotIn,
                    S7Area::ProcessInput,
                    ds.byte_addr(),
                    ds.bit_addr(),
                    ds.len(),
                )
            },
            Area::ProcessOutput(ds) => {
                ItemRequest::new(
                    ds.to_transport_size(),
                    s7_comm::DbNumber::NotIn,
                    S7Area::ProcessOutput,
                    ds.byte_addr(),
                    ds.bit_addr(),
                    ds.len(),
                )
            },
            Area::Merker(ds) => ItemRequest::new(
                ds.to_transport_size(),
                s7_comm::DbNumber::NotIn,
                S7Area::Merker,
                ds.byte_addr(),
                ds.bit_addr(),
                ds.len(),
            ),
            Area::V(ds) => ItemRequest::new(
                ds.to_transport_size(),
                s7_comm::DbNumber::DbNumber(1),
                S7Area::DataBlocks,
                ds.byte_addr(),
                ds.bit_addr(),
                ds.len(),
            ),
            Area::DataBausteine(
                db_number,
                ds,
            ) => ItemRequest::new(
                ds.to_transport_size(),
                s7_comm::DbNumber::DbNumber(
                    *db_number,
                ),
                S7Area::DataBlocks,
                ds.byte_addr(),
                ds.bit_addr(),
                ds.len(),
            ),
        }
    }
}

impl Area {
    pub fn area_data(&self) -> S7Area {
        match &self {
            Area::ProcessInput(_) => {
                S7Area::ProcessInput
            },
            Area::ProcessOutput(_) => {
                S7Area::ProcessOutput
            },
            Area::Merker(_) => S7Area::Merker,
            Area::V(_) => S7Area::DataBlocks,
            Area::DataBausteine(_, _) => {
                S7Area::DataBlocks
            }, /* Area::Counter => {0x1C}
                * Area::Timer => {0x1D} */
        }
    }

    pub fn db_number(&self) -> u16 {
        match self {
            Area::ProcessInput(_) => 0,
            Area::ProcessOutput(_) => 0,
            Area::Merker(_) => 0,
            Area::V(_) => 1,
            Area::DataBausteine(db_number, _) => {
                *db_number
            },
        }
    }
}
impl Deref for Area {
    type Target = DataSizeType;

    fn deref(&self) -> &Self::Target {
        match self {
            Area::ProcessInput(val) => val,
            Area::ProcessOutput(val) => val,
            Area::Merker(val) => val,
            Area::V(val) => val,
            Area::DataBausteine(_, val) => val,
        }
    }
}
#[derive(
    Debug, Copy, Clone, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum BitAddr {
    Addr0 = 0,
    Addr1 = 1,
    Addr2 = 2,
    Addr3 = 3,
    Addr4 = 4,
    Addr5 = 5,
    Addr6 = 6,
    Addr7 = 7,
}
impl TryFrom<u16> for BitAddr {
    type Error = Error;

    fn try_from(
        value: u16,
    ) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Addr0),
            1 => Ok(Self::Addr1),
            2 => Ok(Self::Addr2),
            3 => Ok(Self::Addr3),
            4 => Ok(Self::Addr4),
            5 => Ok(Self::Addr5),
            6 => Ok(Self::Addr6),
            7 => Ok(Self::Addr7),
            val => {
                Err(Error::InvalidBitAddr(val))
            },
        }
    }
}

#[derive(
    Debug, Copy, Clone, Serialize, Deserialize,
)]
pub enum DataSizeType {
    Bit { addr: u16, bit_addr: BitAddr },
    Byte { addr: u16, len: u16 },
}
impl DataSizeType {
    /// 位的偏移位置
    pub fn bit_addr(&self) -> u8 {
        use DataSizeType::*;
        match self {
            Bit { bit_addr, .. } => {
                *bit_addr as u8
            },
            _ => 0x00,
        }
    }

    /// 读取的单位长度
    pub fn len(&self) -> u16 {
        use DataSizeType::*;
        match self {
            Bit { .. } => 1u16,
            Byte { len, .. } => *len,
        }
    }

    pub fn byte_addr(&self) -> u16 {
        use DataSizeType::*;
        match self {
            Bit { addr, .. } => *addr,
            Byte { addr, .. } => *addr,
        }
    }

    pub fn to_transport_size(
        &self,
    ) -> TransportSize {
        match self {
            DataSizeType::Bit { .. } => {
                TransportSize::Bit
            },
            _ => TransportSize::NoBit,
        }
    }
}

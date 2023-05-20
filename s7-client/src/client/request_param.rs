use serde::{Serialize, Deserialize};

use crate::error::S7ConnectError;
use std::ops::Deref;

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
    // Merker,
    /// German thing, means building blocks
    /// This is your storage  : db number,
    /// DataSizeType
    DataBausteine(u16, DataSizeType),
    V(DataSizeType), /* Counter,
                      * Timer, */
}
impl Area {
    pub fn area_data(&self) -> u8 {
        match &self {
            Area::ProcessInput(_) => 0x81,
            Area::ProcessOutput(_) => 0x82,
            Area::V(_) => 0x84,
            // Area::Merker => {0x83}
            Area::DataBausteine(_, _) => 0x84, /* Area::Counter => {0x1C}
                                                * Area::Timer => {0x1D} */
        }
    }

    pub fn db_number(&self) -> u16 {
        match self {
            Area::ProcessInput(_) => 0,
            Area::ProcessOutput(_) => 0,
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
    type Error = S7ConnectError;

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
            val => Err(
                S7ConnectError::InvalidBitAddr(
                    val,
                ),
            ),
        }
    }
}

#[derive(
    Debug, Copy, Clone, Serialize, Deserialize,
)]
pub enum DataSizeType {
    Bit { addr: u16, bit_addr: BitAddr },
    Byte { addr: u16, len: u16 },
    Char { addr: u16, len: u16 },
    Word { addr: u16, len: u16 },
    Int { addr: u16, len: u16 },
    DWord { addr: u16, len: u16 },
    DInt { addr: u16, len: u16 },
    Real { addr: u16, len: u16 },
    Counter { addr: u16, len: u16 },
    Timer { addr: u16, len: u16 },
}
impl DataSizeType {
    /// 类型对应的字节长度
    pub fn length(&self) -> u16 {
        use DataSizeType::*;
        match self {
            Bit { .. }
            | Byte { .. }
            | Char { .. } => 1,
            Word { .. }
            | Int { .. }
            | Counter { .. }
            | Timer { .. } => 2,
            DWord { .. }
            | DInt { .. }
            | Real { .. } => 4,
        }
    }

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
            Char { len, .. } => *len,
            Word { len, .. } => *len,
            Int { len, .. } => *len,
            DWord { len, .. } => *len,
            DInt { len, .. } => *len,
            Real { len, .. } => *len,
            Counter { len, .. } => *len,
            Timer { len, .. } => *len,
        }
    }

    /// 用于返回后的byte长度 = 读取长度 *
    /// 单位字节数
    pub fn byte_len(&self) -> usize {
        (self.len() * self.length()) as usize
    }

    pub fn addr(&self) -> [u8; 3] {
        use DataSizeType::*;
        let byte_addr = match self {
            Bit { addr, .. } => *addr,
            Byte { addr, .. } => *addr,
            Char { addr, .. } => *addr,
            Word { addr, .. } => *addr,
            Int { addr, .. } => *addr,
            DWord { addr, .. } => *addr,
            DInt { addr, .. } => *addr,
            Real { addr, .. } => *addr,
            Counter { addr, .. } => *addr,
            Timer { addr, .. } => *addr,
        };
        let address = ((byte_addr as u32) << 3)
            + self.bit_addr() as u32;
        [
            ((address & 0x00FF0000) >> 16) as u8,
            ((address & 0x0000FF00) >> 8) as u8,
            (address & 0x000000FF) as u8,
        ]
    }

    pub fn data(&self) -> u8 {
        use DataSizeType::*;
        match self {
            Bit { .. } => 0x01,
            Byte { .. } => 0x02,
            Char { .. } => 0x03,
            Word { .. } => 0x04,
            Int { .. } => 0x05,
            DWord { .. } => 0x06,
            DInt { .. } => 0x07,
            Real { .. } => 0x08,
            Counter { .. } => 0x1C,
            Timer { .. } => 0x1D,
        }
    }
}

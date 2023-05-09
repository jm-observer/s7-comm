use crate::builder::ConnectBuilder;
use crate::error::*;
use crate::DtDataBuilder;
use bytes::{Buf, BufMut, BytesMut};
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq)]
pub struct CoptFrame<F: Debug + Eq + PartialEq> {
    pub pdu_type: PduType<F>,
}

impl<F: Debug + Eq + PartialEq> CoptFrame<F> {
    pub fn builder_of_dt_data(payload: F) -> DtDataBuilder<F> {
        DtDataBuilder::new(payload)
    }

    pub fn builder_of_connect() -> ConnectBuilder {
        ConnectBuilder::default()
    }

    pub fn length(&self) -> u8 {
        self.pdu_type.length()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum PduType<F: Debug + Eq + PartialEq> {
    /// 0x0e
    ConnectRequest(ConnectComm),
    /// 0x0d
    ConnectConfirm(ConnectComm),
    /// 0x0f
    DtData(DtData<F>),
}

impl<F: Debug + Eq + PartialEq> PduType<F> {
    pub fn length(&self) -> u8 {
        match self {
            PduType::ConnectRequest(conn) => conn.length(),
            PduType::ConnectConfirm(conn) => conn.length(),
            PduType::DtData(_) => 2,
        }
    }
}
#[derive(Debug, Eq, PartialEq)]
pub struct DtData<F: Debug + Eq + PartialEq> {
    pub tpdu_number: u8,
    pub last_data_unit: bool,
    pub payload: F,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ConnectComm {
    pub destination_ref: [u8; 2],
    pub source_ref: [u8; 2],
    pub class: u8,
    pub extended_formats: bool,
    pub no_explicit_flow_control: bool,
    pub parameters: Vec<Parameter>,
}

impl ConnectComm {
    pub fn length(&self) -> u8 {
        5 + self.parameters.iter().fold(0, |x, item| x + item.length())
    }
    pub(crate) fn decode(src: &mut BytesMut) -> Result<Self> {
        if src.len() < 5 {
            return Err(Error::Error("data not enough".to_string()));
        }
        let destination_ref = [src.get_u8(), src.get_u8()];
        let source_ref = [src.get_u8(), src.get_u8()];
        let merge = src.get_u8();
        let class = merge >> 4;
        let extended_formats = merge << 6 >> 7 > 0;
        let no_explicit_flow_control = merge & 1 > 0;

        let mut parameters = Vec::new();
        while let Some(parameter) = Parameter::decode(src)? {
            parameters.push(parameter);
        }
        Ok(Self {
            destination_ref,
            source_ref,
            class,
            extended_formats,
            no_explicit_flow_control,
            parameters,
        })
    }
    pub(crate) fn encode(&self, dst: &mut BytesMut) {
        dst.put_slice(self.destination_ref.as_ref());
        dst.put_slice(self.source_ref.as_ref());

        let merge = self.class << 4
            & if self.extended_formats { 2 } else { 0 }
            & if self.no_explicit_flow_control { 1 } else { 0 };
        dst.put_u8(merge);
        self.parameters.iter().for_each(|x| x.encode(dst));
    }
}
#[derive(Debug, Eq, PartialEq)]
pub enum Parameter {
    /// 0xc0
    TpduSize(Vec<u8>),
    /// 0xc1
    SrcTsap(Vec<u8>),
    /// 0xc2
    DstTsap(Vec<u8>),
}

impl Parameter {
    pub fn length(&self) -> u8 {
        match self {
            Parameter::TpduSize(data) => 2 + data.len() as u8,
            Parameter::SrcTsap(data) => 2 + data.len() as u8,
            Parameter::DstTsap(data) => 2 + data.len() as u8,
        }
    }
    fn decode(dst: &mut BytesMut) -> Result<Option<Self>> {
        if dst.len() == 0 {
            return Ok(None);
        }
        let (Some(ty), Some(length)) = (dst.get(0), dst.get(1)) else {
            return Err(Error::Error("data not enough".to_string()));
        };
        let length = (length + 2) as usize;
        let ty = *ty;
        if dst.len() < length {
            return Err(Error::Error("data not enough".to_string()));
        }
        let data = dst.split_to(length).split_off(2);
        match ty {
            0xc0 => Ok(Some(Self::TpduSize(data.to_vec()))),
            0xc1 => Ok(Some(Self::TpduSize(data.to_vec()))),
            0xc2 => Ok(Some(Self::TpduSize(data.to_vec()))),
            _ => {
                return Err(Error::Error(format!("not support parameter: {}", ty)));
            }
        }
    }
    fn encode(&self, dst: &mut BytesMut) {
        match self {
            Parameter::TpduSize(data) => {
                dst.put_u8(0xc0);
                dst.put_u8(data.len() as u8);
                dst.extend_from_slice(data.as_ref())
            }
            Parameter::SrcTsap(data) => {
                dst.put_u8(0xc1);
                dst.put_u8(data.len() as u8);
                dst.extend_from_slice(data.as_ref())
            }
            Parameter::DstTsap(data) => {
                dst.put_u8(0xc2);
                dst.put_u8(data.len() as u8);
                dst.extend_from_slice(data.as_ref())
            }
        }
    }
}

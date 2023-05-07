use tokio_util::codec::{Decoder, Encoder};
use bytes::{Buf, BufMut, BytesMut};
use anyhow::{bail, Error, Result};

pub struct CoptEncoder<E>(E);
pub struct CoptDecoder<D>(D);


pub struct CoptFrame<F> {
    pub pdu_type: PduType<F>,
}

impl <F> CoptFrame<F> {
    pub fn length(&self) -> u8 {
        self.pdu_type.length()
    }
}

pub enum PduType<F> {
    /// 0x0e
    ConnectRequest(ConnectComm),
    /// 0x0d
    ConnectConfirm(ConnectComm),
    /// 0x0f
    DtData(DtData<F>)
}

impl<F> PduType<F> {
    pub fn length(&self) -> u8 {
        match self {
            PduType::ConnectRequest(conn) => {
                conn.length()
            }
            PduType::ConnectConfirm(conn) => {
                conn.length()
            }
            PduType::DtData(_) => {2}
        }
    }
}

pub struct DtData<F> {
    tpdu_number: u8,
    last_data_unit: bool,
    payload: F,
}

pub struct ConnectComm {
    pub destination_ref: [u8; 2],
    pub source_ref: [u8; 2],
    pub class: u8,
    pub extended_formats: bool,
    pub no_explicit_flow_control: bool,
    pub parameters: Vec<Parameter>
}

impl ConnectComm {
    pub fn length(&self) -> u8 {
        5 + self.parameters.iter().fold(0, |x, item| {
            x + item.length()
        })
    }
    fn decode(src: &mut BytesMut) -> Result<Self> {
        if src.len() < 5 {
            bail!("data not enough");
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
        };
        Ok(Self {
            destination_ref,
            source_ref,
            class,
            extended_formats,
            no_explicit_flow_control,
            parameters,
        })

    }
        fn encode(&self, dst: &mut BytesMut) {
        dst.put_slice(self.destination_ref.as_ref());
        dst.put_slice(self.source_ref.as_ref());

        let merge = self.class << 4 & if self.extended_formats {
            2
        } else {
            0
        } & if self.no_explicit_flow_control {
            1
        } else {
            0
        };
        dst.put_u8(merge);
        self.parameters.iter().for_each(|x| x.encode(dst));
    }
}

pub enum  Parameter {
    /// 0xc0
    TpduSize(Vec<u8>),
    /// 0xc1
    SrcTsap(Vec<u8>),
    /// 0xc2
    DstTsap(Vec<u8>)
}

impl Parameter {
    pub fn length(&self) -> u8 {
        match self {
            Parameter::TpduSize(data) => { 2 + data.len() as u8 }
            Parameter::SrcTsap(data) => { 2 + data.len() as u8 }
            Parameter::DstTsap(data) => { 2 + data.len() as u8 }
        }
    }
    fn decode(dst: &mut BytesMut) -> Result<Option<Self>> {
        if dst.len() == 0 {
            return Ok(None);
        }
        let (Some(ty), Some(length)) = (dst.get(0), dst.get(1)) else {
            bail!("data not enough");
        };
        let length = (length + 2) as usize;
        if dst.len() < length {
            bail!("data not enough");
        }
        let data = dst.split_to(length).split_off(2);
        match *ty {
            0xc0 => {
                Ok(Some(Self::TpduSize(data.to_vec())))
            }
            0xc1 => {
                Ok(Some(Self::TpduSize(data.to_vec())))
            }
            0xc2 => {
                Ok(Some(Self::TpduSize(data.to_vec())))
            }
            _ => {
                bail!("not support parameter: {}", ty);
            }
        }
    }
    fn encode(&self, dst: &mut BytesMut) {
        match self {
            Parameter::TpduSize(data) => {
                dst.put_u8(0xc0);
                dst.put_u8(data.len() as u8);
                dst.extend_from_slice(data.as_ref())}
            Parameter::SrcTsap(data) => {
                dst.put_u8(0xc1);
                dst.put_u8(data.len() as u8);
                dst.extend_from_slice(data.as_ref())}
            Parameter::DstTsap(data) => {
                dst.put_u8(0xc2);
                dst.put_u8(data.len() as u8);
                dst.extend_from_slice(data.as_ref())}
        }
    }
}

impl <F, E: Encoder<F>> Encoder<CoptFrame<F>> for CoptEncoder<E> where <E as Encoder<F>>::Error: std::error::Error + Send + Sync + 'static {
    type Error = Error;

    fn encode(&mut self, item: CoptFrame<F>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_u8(item.length());
        match item {
            PduType::ConnectRequest(conn) => {
                dst.put_u8(0x0e);
                conn.encode(dst);
                Ok(())
            }
            PduType::ConnectConfirm(conn) => {
                dst.put_u8(0x0d);
                conn.encode(dst);
                Ok(())
            }
            PduType::DtData(conn) => {
                dst.put_u8(0x0f);
                let merge = conn.tpdu_number >> 1 & if conn.last_data_unit {
                    0b1000000
                } else {
                    0
                };
                dst.put_u8(merge);
                Ok(self.0.encode(conn.payload, dst)?)
            }
        }
    }
}

impl <F, D: Decoder<Item=F>> Decoder for CoptDecoder<D> where <D as Decoder>::Error: std::error::Error + Send + Sync + 'static  {
    type Item = CoptFrame<F>;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let (Some(length), Some(pdu_type)) = (src.get(0), src.get(1)) else {
            return Ok(None)
        };
        let length = *length as usize + 1;
        if src.len() < length || length < 2 {
            return Ok(None)
        };
        match *pdu_type {
            0x0e => {
                let mut src = src.split_to(length).split_off(2);
                Ok(Some(CoptFrame {
                    pdu_type: PduType::ConnectRequest(ConnectComm::decode(&mut src)?),
                }))
            }
            0x0d => {
                let mut src = src.split_to(length).split_off(2);
                Ok(Some(CoptFrame {
                    pdu_type: PduType::ConnectConfirm(ConnectComm::decode(&mut src)?),
                }))
            }
            0x0f => {
                let mut sub_src = src.clone().split_off(length);
                let pre_length = sub_src.len();
                let Some(f) = self.0.decode(&mut sub_src)? else {
                    bail!("decode fail")
                };
                let sub_length = pre_length - sub_src.len();
                let mut src = src.split_to(length + sub_length + 1).split_off(2);
                let merge = src.get_u8();
                let tpdu_number = merge & 0b0111_1111;
                let last_data_unit = merge & 0b1000_0000 > 0;
                Ok(Some(CoptFrame {
                    pdu_type: PduType::DtData(DtData {
                        tpdu_number,
                        last_data_unit,
                        payload: f,
                    }),
                }))
            }
            _ => {
                bail!("not support pdu type: {}", pdu_type);
            }
        }
    }
}
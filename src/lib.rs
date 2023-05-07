pub mod builder;
mod packet;

use crate::packet::{AckData, Frame, Header, HearderAckData, Job};
use anyhow::{bail, Error};
use bytes::{BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

pub struct S7CommEncoder;
pub struct S7CommDecoder;

impl Encoder<Frame> for S7CommEncoder {
    type Error = Error;

    fn encode(&mut self, item: Frame, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match item {
            Frame::Job { header, job } => {
                let Header {
                    protocol_id,
                    reserved,
                    pdu_ref,
                    parameter_len,
                    data_len,
                } = header;
                dst.put_u8(protocol_id);
                dst.put_u8(0x01);
                dst.extend_from_slice(reserved.to_be_bytes().as_slice());
                dst.extend_from_slice(pdu_ref.to_be_bytes().as_slice());
                dst.extend_from_slice(parameter_len.to_be_bytes().as_slice());
                dst.extend_from_slice(data_len.to_be_bytes().as_slice());
                match job {
                    Job::SetupCommunication(data) => {
                        dst.put_u8(0xf0);
                        data.encode(dst);
                    }
                    Job::WriteVar(data) => {
                        dst.put_u8(0x05);
                        data.encode(dst);
                    }
                    Job::ReadVar(data) => {
                        dst.put_u8(0x04);
                        data.encode(dst);
                    }
                }
            }
            Frame::AckData { header, ack_data } => {
                let HearderAckData {
                    protocol_id,
                    reserved,
                    pdu_ref,
                    parameter_len,
                    data_len,
                    error_class,
                    error_code,
                } = header;
                dst.put_u8(protocol_id);
                dst.put_u8(0x03);
                dst.extend_from_slice(reserved.to_be_bytes().as_slice());
                dst.extend_from_slice(pdu_ref.to_be_bytes().as_slice());
                dst.extend_from_slice(parameter_len.to_be_bytes().as_slice());
                dst.extend_from_slice(data_len.to_be_bytes().as_slice());
                dst.put_u8(error_class);
                dst.put_u8(error_code);
                match ack_data {
                    AckData::SetupCommunication(data) => {
                        dst.put_u8(0xf0);
                        data.encode(dst);
                    }
                    AckData::WriteVar(data) => {
                        dst.put_u8(0x05);
                        data.encode(dst);
                    }
                    AckData::ReadVar(data) => {
                        dst.put_u8(0x04);
                        data.encode(dst);
                    }
                }
            }
        }
        Ok(())
    }
}

impl Decoder for S7CommDecoder {
    type Item = Frame;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 10 {
            return Ok(None);
        }
        let Some(rosctr) = src.get(1) else {
            unreachable!()
        };
        let (Some(parameter_0), Some(parameter_1)) = (src.get(1), src.get(1)) else {
            unreachable!()
        };
        let (Some(data_0), Some(data_1)) = (src.get(1), src.get(1)) else {
            unreachable!()
        };

        let parameter_length = u16::from_be_bytes([*parameter_0, *parameter_1]);
        let data_length = u16::from_be_bytes([*data_0, *data_1]);
        match *rosctr {
            1 => {
                // job
                if src.len() < (10 + parameter_length + data_length) as usize {
                    return Ok(None);
                }
                let header = Header::decode(src);
                let job = Job::decode(src)?;
                Ok(Some(Frame::Job { header, job }))
            }
            3 => {
                // ack data
                if src.len() < (12 + parameter_length + data_length) as usize {
                    return Ok(None);
                }
                let header = HearderAckData::decode(src);
                let ack_data = AckData::decode(src)?;
                Ok(Some(Frame::AckData { header, ack_data }))
            }
            _ => {
                bail!("not support rosctr: {}", rosctr);
            }
        }
    }
}

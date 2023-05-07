use anyhow::{bail, Error, Result};
use bytes::{Buf, BufMut, BytesMut};
use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
/// more info: https://github.com/wireshark/wireshark/blob/master/epan/dissectors/packet-s7comm.c
use tokio_util::codec::{Decoder, Encoder};

pub struct S7CommEncoder;
pub struct S7CommDecoder;

pub struct Hearder {
    /// 0x32?
    protocol_id: u8,
    reserved: u16,
    pdu_ref: u16,
    parameter_len: u16,
    data_len: u16,
}

impl Hearder {
    fn decode(src: &mut BytesMut) -> Self {
        let protocol_id = src.get_u8();
        src.get_u8();
        let reserved = src.get_u16();
        let pdu_ref = src.get_u16();
        let parameter_len = src.get_u16();
        let data_len = src.get_u16();
        Self {
            protocol_id,
            reserved,
            pdu_ref,
            parameter_len,
            data_len,
        }
    }
}

pub struct HearderAckData {
    /// 0x32?
    protocol_id: u8,
    reserved: u16,
    pdu_ref: u16,
    parameter_len: u16,
    data_len: u16,
    error_class: u8,
    error_code: u8,
}

impl HearderAckData {
    fn decode(src: &mut BytesMut) -> Self {
        let protocol_id = src.get_u8();
        src.get_u8();
        let reserved = src.get_u16();
        let pdu_ref = src.get_u16();
        let parameter_len = src.get_u16();
        let data_len = src.get_u16();
        let error_class = src.get_u8();
        let error_code = src.get_u8();
        Self {
            protocol_id,
            reserved,
            pdu_ref,
            parameter_len,
            data_len,
            error_class,
            error_code,
        }
    }
}

pub enum Frame {
    /// 0x01
    Job { header: Hearder, job: Job },
    /// 0x03
    AckData {
        header: HearderAckData,
        ack_data: AckData,
    },
}

// #[derive(IntoPrimitive, FromPrimitive)]
// #[repr(u8)]
// pub enum Rosctr {
//     /// 0x01
//     Job,
//     /// 0x02
//     Ack = 0x02,
//     /// 0x03
//     AckData = 0x03,
// }

pub enum Job {
    /// 0xf0
    SetupCommunication(SetupCommunication),
    /// 0x05
    WriteVar(WriteVarJob),
    /// 0x04
    ReadVar(ReadVarJob),
}

impl Job {
    fn decode(src: &mut BytesMut) -> Result<Self> {
        let function = src.get_u8();
        match function {
            0x04 => {
                let count = src.get_u8();
                let mut parameters_item = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    parameters_item.push(ItemRequest::decode(src)?);
                }
                Ok(Self::ReadVar(ReadVarJob {
                    count: 0,
                    parameters_item,
                }))
            }
            0x05 => {
                let count = src.get_u8();
                let mut parameters_item = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    parameters_item.push(ItemRequest::decode(src)?);
                }
                let mut data_item = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    data_item.push(DataItemVal::decode(src)?);
                }
                Ok(Self::WriteVar(WriteVarJob {
                    count: 0,
                    parameters_item,
                    data_item,
                }))
            }
            0xf0 => {
                let data = SetupCommunication::decode(src)?;
                Ok(Self::SetupCommunication(data))
            }
            _ => {
                bail!("not support function: {}", function);
            }
        }
    }
}

pub enum AckData {
    /// 0xf0
    SetupCommunication(SetupCommunication),
    /// 0x05
    WriteVar(WriteVarAckData),
    /// 0x04
    ReadVar(ReadVarAckData),
}

impl AckData {
    fn decode(src: &mut BytesMut) -> Result<Self> {
        let function = src.get_u8();
        match function {
            0x04 => {
                let count = src.get_u8();
                let mut data_item = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    data_item.push(DataItemVal::decode(src)?);
                }
                Ok(Self::ReadVar(ReadVarAckData {
                    count: 0,
                    data_item,
                }))
            }
            0x05 => {
                let count = src.get_u8();
                let mut parameters_item = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    parameters_item.push(ItemRequest::decode(src)?);
                }
                let mut data_item = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    data_item.push(DataItemWriteResponse::decode(src)?);
                }
                Ok(Self::WriteVar(WriteVarAckData {
                    count: 0,
                    data_item,
                }))
            }
            0xf0 => {
                let data = SetupCommunication::decode(src)?;
                Ok(Self::SetupCommunication(data))
            }
            _ => {
                bail!("not support function: {}", function);
            }
        }
    }
}
//////////////////////////////////////

pub struct WriteVarJob {
    count: u8,
    parameters_item: Vec<ItemRequest>,
    data_item: Vec<DataItemVal>,
}

impl WriteVarJob {
    fn encode(self, dst: &mut BytesMut) {
        dst.put_u8(self.count);
        self.parameters_item.into_iter().for_each(|x| x.encode(dst));
        self.data_item.into_iter().for_each(|x| x.encode(dst));
    }
}
pub struct WriteVarAckData {
    count: u8,
    data_item: Vec<DataItemWriteResponse>,
}
impl WriteVarAckData {
    fn encode(self, dst: &mut BytesMut) {
        dst.put_u8(self.count);
        self.data_item.into_iter().for_each(|x| x.encode(dst));
    }
}

pub struct ReadVarJob {
    count: u8,
    parameters_item: Vec<ItemRequest>,
}
impl ReadVarJob {
    fn encode(self, dst: &mut BytesMut) {
        dst.put_u8(self.count);
        self.parameters_item.into_iter().for_each(|x| x.encode(dst));
    }
}

pub struct ReadVarAckData {
    count: u8,
    data_item: Vec<DataItemVal>,
}
impl ReadVarAckData {
    fn encode(self, dst: &mut BytesMut) {
        dst.put_u8(self.count);
        self.data_item.into_iter().for_each(|x| x.encode(dst));
    }
}
//////////////////////////////////////

// pub enum Parameter {
//     /// 0xf0
//     SetupCommunication(SetupCommunication),
//     /// 0x05
//     WriteVarRequest {
//         count: u8,
//         item_request: ItemRequest,
//     },
//     /// 0x04
//     ReadVarRequest {
//         count: u8,
//         item_request: ItemRequest,
//     },
// }
//
// pub enum Data {
//     WriteVarResponse { return_code: ReturnCode },
//     ReadVarResponse(DataItemVal),
// }
//
// impl Parameter {
//     fn encode(self, dst: &mut BytesMut) {
//         match self {
//             Parameter::SetupCommunication(data) => {
//                 dst.put_u8(0xf0);
//                 data.encode(dst);
//             }
//             Parameter::WriteVarRequest {
//                 count,
//                 item_request,
//                 data_item,
//             } => {
//                 dst.put_u8(0x05);
//                 dst.put_u8(count);
//                 item_request.encode(dst);
//                 data_item.encode(dst);
//             }
//             Parameter::WriteVarResponse { count, data_item } => {
//                 dst.put_u8(0x05);
//                 dst.put_u8(count);
//                 data_item.encode(dst);
//             }
//             Parameter::ReadVarRequest {
//                 count,
//                 item_request,
//             } => {
//                 dst.put_u8(0x04);
//                 dst.put_u8(count);
//                 item_request.encode(dst);
//             }
//             Parameter::ReadVarResponse { count, data_item } => {
//                 dst.put_u8(0x04);
//                 dst.put_u8(count);
//                 data_item.encode(dst);
//             }
//         }
//     }
// }

pub struct SetupCommunication {
    reserved: u8,
    max_amq_calling: u16,
    max_amq_called: u16,
    pdu_length: u16,
}

impl SetupCommunication {
    fn len() -> usize {
        7
    }
    fn encode(self, dst: &mut BytesMut) {
        dst.put_u8(self.reserved);
        dst.extend_from_slice(self.max_amq_calling.to_be_bytes().as_slice());
        dst.extend_from_slice(self.max_amq_called.to_be_bytes().as_slice());
        dst.extend_from_slice(self.pdu_length.to_be_bytes().as_slice());
    }

    fn decode(src: &mut BytesMut) -> Result<Self> {
        if src.len() < Self::len() {
            bail!("data of SetupCommunication not enough");
        }
        let reserved = src.get_u8();
        let max_amq_calling = src.get_u16();
        let max_amq_called = src.get_u16();
        let pdu_length = src.get_u16();
        Ok(Self {
            reserved,
            max_amq_calling,
            max_amq_called,
            pdu_length,
        })
    }
}

pub struct ItemRequest {
    /// always = 0x12?
    variable_specification: u8,
    follow_length: u8,
    syntax_id: Syntax,
    transport_size_type: TransportSize,
    length: u16,
    db_number: DbNumber,
    area: Area,
    address: Address,
}

impl ItemRequest {
    fn encode(self, dst: &mut BytesMut) {
        dst.put_u8(self.variable_specification);
        dst.put_u8(self.follow_length);
        dst.put_u8(self.syntax_id.into());
        dst.put_u8(self.transport_size_type.into());
        dst.extend_from_slice(self.length.to_be_bytes().as_slice());
        dst.put_u16(self.db_number.into());
        dst.put_u8(self.area.into());
        dst.extend_from_slice(self.address.to_bytes().as_slice());
    }

    fn decode(src: &mut BytesMut) -> Result<Self> {
        if src.len() < 12 {
            bail!("todo");
        }
        let variable_specification = src.get_u8();
        let follow_length = src.get_u8();
        let syntax_id = Syntax::from(src.get_u8());
        let transport_size_type = TransportSize::from(src.get_u8());
        let length = src.get_u16();
        let db_number = DbNumber::from(src.get_u16());
        let area = Area::from(src.get_u8());
        let address = Address::from_bytes(src.get_u8(), src.get_u8(), src.get_u8());
        Ok(Self {
            variable_specification,
            follow_length,
            syntax_id,
            transport_size_type,
            length,
            db_number,
            area,
            address,
        })
    }
}

pub struct DataItemWriteResponse {
    return_code: ReturnCode,
}

impl DataItemWriteResponse {
    fn encode(self, dst: &mut BytesMut) {
        dst.put_u8(self.return_code.into());
    }

    fn decode(src: &mut BytesMut) -> Result<Self> {
        if src.len() == 0 {
            bail!("todo");
        }
        Ok(Self {
            return_code: ReturnCode::try_from(src.get_u8())?,
        })
    }
}

pub struct DataItemVal {
    return_code: ReturnCode,
    /// always = 0x04?
    transport_size_type: TransportSize,
    length: u16,
    data: Vec<u8>,
}

impl DataItemVal {
    fn encode(self, dst: &mut BytesMut) {
        dst.put_u8(self.return_code.into());
        dst.put_u8(self.transport_size_type.into());
        dst.extend_from_slice(self.length.to_be_bytes().as_slice());
        dst.extend_from_slice(self.data.as_slice());
    }

    fn decode(src: &mut BytesMut) -> Result<Self> {
        if src.len() < 4 {
            bail!("todo")
        }
        let return_code = ReturnCode::try_from(src.get_u8())?;
        let transport_size_type = TransportSize::from(src.get_u8());
        let length = src.get_u16();
        if src.len() < length as usize {
            bail!("todo")
        }
        let mut data = Vec::with_capacity(length as usize);
        for _ in 0..length {
            data.push(src.get_u8())
        }
        Ok(Self {
            return_code,
            transport_size_type,
            length,
            data,
        })
    }
}

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ReturnCode {
    /// 0
    Reserved = 0,
    /// 0xff
    Success = 0xff,
}
#[derive(IntoPrimitive, FromPrimitive)]
/// ?
#[repr(u8)]
pub enum TransportSize {
    Bit = 0x01,
    Byte = 0x02,
    Char = 0x03,
    Word = 0x04,
    Int = 0x05,
    DWord = 0x06,
    DInt = 0x07,
    Real = 0x08,
    Counter = 0x1C,
    Timer = 0x1D,
    #[num_enum(catch_all)]
    NotSupport(u8),
}
#[derive(IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum Area {
    ProcessInput = 0x81,
    ProcessOutput = 0x82,
    DataBlocks = 0x84,
    Merker = 0x83,
    Counter = 0x1c,
    Timer = 0x1d,
    #[num_enum(catch_all)]
    NotSupport(u8),
}

#[derive(IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum Syntax {
    S7Any = 0x10,
    #[num_enum(catch_all)]
    NotSupport(u8),
}
#[derive(IntoPrimitive, FromPrimitive)]
#[repr(u16)]
pub enum DbNumber {
    NotIn = 0,
    #[num_enum(catch_all)]
    DbNumber(u16),
}

pub struct Address {
    byte_addr: u16,
    bit_addr: u8,
}

impl Address {
    pub fn to_bytes(&self) -> [u8; 3] {
        let [byte_0, byte_1] = self.byte_addr.to_be_bytes();
        [
            byte_0 >> 5,
            byte_0 << 3 | byte_1 >> 5,
            byte_1 << 3 | self.bit_addr,
        ]
    }

    pub fn from_bytes(index_0: u8, index_1: u8, index_2: u8) -> Self {
        let index_0 = index_0 << 5 | index_1 >> 3;
        let index_1 = index_1 << 5 | index_2 >> 3;

        let byte_addr = u16::from_be_bytes([index_0, index_1]);
        let bit_addr = index_2 & 0b0000_0111;
        Self {
            byte_addr,
            bit_addr,
        }
    }
}

impl Encoder<Frame> for S7CommEncoder {
    type Error = Error;

    fn encode(&mut self, item: Frame, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match item {
            Frame::Job { header, job } => {
                let Hearder {
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
                let header = Hearder::decode(src);
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

#[cfg(test)]
mod test {
    use crate::Address;

    #[test]
    fn check_address() {
        let addr = Address {
            byte_addr: 0b0000000100101100,
            bit_addr: 0,
        };
        assert_eq!(addr.byte_addr, 300);
        assert_eq!(addr.to_bytes(), [0, 9, 0x60])
    }
}

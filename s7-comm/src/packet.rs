use crate::{builder::*, error::*};
use bytes::{Buf, BufMut, BytesMut};
use num_enum::{
    FromPrimitive, IntoPrimitive,
    TryFromPrimitive,
};
/// more info: https://github.com/wireshark/wireshark/blob/master/epan/dissectors/packet-s7comm.c

#[derive(Debug, Eq, PartialEq)]
pub enum Frame {
    /// 0x01
    Job { header: Header, job: Job },
    /// 0x03
    AckData {
        header: HearderAckData,
        ack_data: AckData,
    },
}

impl Frame {
    pub fn job_setup(
        pdu_ref: u16,
    ) -> FrameJobSetupBuilder {
        FrameJobSetupBuilder::default()
            .pdu_ref(pdu_ref)
    }

    pub fn job_write_var(
        pdu_ref: u16,
    ) -> FrameJobWriteVarBuilder {
        FrameJobWriteVarBuilder::default()
            .pdu_ref(pdu_ref)
    }

    pub fn job_read_var(
        pdu_ref: u16,
    ) -> FrameJobReadVarBuilder {
        FrameJobReadVarBuilder::default()
            .pdu_ref(pdu_ref)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Header {
    /// 0x32?
    pub protocol_id: u8,
    pub reserved: u16,
    pub pdu_ref: u16,
    pub parameter_len: u16,
    pub data_len: u16,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            protocol_id: 0x32,
            reserved: 0,
            pdu_ref: 0x0400,
            parameter_len: 0,
            data_len: 0,
        }
    }
}

impl Header {
    pub fn init(
        pdu_ref: u16,
        parameter_len: u16,
        data_len: u16,
    ) -> Self {
        Self {
            protocol_id: 0x32,
            reserved: 0,
            pdu_ref,
            parameter_len,
            data_len,
        }
    }

    pub(crate) fn decode(
        src: &mut BytesMut,
    ) -> Self {
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

#[derive(Debug, Eq, PartialEq)]
pub struct HearderAckData {
    /// 0x32?
    pub(crate) protocol_id: u8,
    pub(crate) reserved: u16,
    pub(crate) pdu_ref: u16,
    pub(crate) parameter_len: u16,
    pub(crate) data_len: u16,
    pub(crate) error_class: u8,
    pub(crate) error_code: u8,
}

impl HearderAckData {
    pub fn init(
        pdu_ref: u16,
        parameter_len: u16,
        data_len: u16,
        error_class: u8,
        error_code: u8,
    ) -> Self {
        Self {
            protocol_id: 0x32,
            reserved: 0,
            pdu_ref,
            parameter_len,
            data_len,
            error_class,
            error_code,
        }
    }

    pub(crate) fn decode(
        src: &mut BytesMut,
    ) -> Self {
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
#[derive(Debug, Eq, PartialEq)]
pub enum Job {
    /// 0xf0
    SetupCommunication(SetupCommunication),
    /// 0x05
    WriteVar(WriteVarJob),
    /// 0x04
    ReadVar(ReadVarJob),
}

impl Job {
    pub(crate) fn decode(
        src: &mut BytesMut,
    ) -> Result<Self> {
        let function = src.get_u8();
        match function {
            0x04 => {
                let count = src.get_u8();
                let mut parameters_item =
                    Vec::with_capacity(
                        count as usize,
                    );
                for _ in 0..count {
                    parameters_item.push(
                        ItemRequest::decode(src)?,
                    );
                }
                Ok(Self::ReadVar(ReadVarJob {
                    count: 0,
                    parameters_item,
                }))
            },
            0x05 => {
                let count = src.get_u8();
                let mut parameters_item =
                    Vec::with_capacity(
                        count as usize,
                    );
                for _ in 0..count {
                    parameters_item.push(
                        ItemRequest::decode(src)?,
                    );
                }
                let mut data_item =
                    Vec::with_capacity(
                        count as usize,
                    );
                for _ in 0..count {
                    data_item.push(
                        DataItemVal::decode(src)?,
                    );
                }
                Ok(Self::WriteVar(WriteVarJob {
                    count: 0,
                    parameters_item,
                    data_item,
                }))
            },
            0xf0 => {
                let data =
                    SetupCommunication::decode(
                        src,
                    )?;
                Ok(Self::SetupCommunication(data))
            },
            _ => Err(Error::Error(format!(
                "not support function: {}",
                function
            ))),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum AckData {
    /// 0xf0
    SetupCommunication(SetupCommunication),
    /// 0x05
    WriteVar(WriteVarAckData),
    /// 0x04
    ReadVar(ReadVarAckData),
}

impl AckData {
    pub(crate) fn decode(
        src: &mut BytesMut,
    ) -> Result<Self> {
        let function = src.get_u8();
        match function {
            0x04 => {
                let count = src.get_u8();
                let mut data_item =
                    Vec::with_capacity(
                        count as usize,
                    );
                for _ in 0..count {
                    data_item.push(
                        DataItemVal::decode(src)?,
                    );
                }
                Ok(Self::ReadVar(
                    ReadVarAckData {
                        count,
                        data_item,
                    },
                ))
            },
            0x05 => {
                let count = src.get_u8();
                // let mut parameters_item =
                // Vec::with_capacity(count as
                // usize);
                // for _ in 0..count {
                //     parameters_item.
                // push(ItemRequest::decode(src)?
                // ); }
                let mut data_item =
                    Vec::with_capacity(
                        count as usize,
                    );
                for _ in 0..count {
                    data_item.push(DataItemWriteResponse::decode(src)?);
                }
                Ok(Self::WriteVar(
                    WriteVarAckData {
                        count,
                        data_item,
                    },
                ))
            },
            0xf0 => {
                let data =
                    SetupCommunication::decode(
                        src,
                    )?;
                Ok(Self::SetupCommunication(data))
            },
            _ => Err(Error::Error(format!(
                "not support function: {}",
                function
            ))),
        }
    }
}
//////////////////////////////////////

#[derive(Default, Debug, Eq, PartialEq)]
pub struct WriteVarJob {
    count: u8,
    parameters_item: Vec<ItemRequest>,
    data_item: Vec<DataItemVal>,
}

impl WriteVarJob {
    pub fn bytes_len_data(&self) -> u16 {
        self.data_item
            .iter()
            .fold(0, |len, x| len + x.bytes_len())
    }

    pub fn bytes_len_parameter(&self) -> u16 {
        self.parameters_item
            .iter()
            .fold(2, |len, x| len + x.bytes_len())
    }

    pub fn add_item(
        &mut self,
        x: (ItemRequest, DataItemVal),
    ) {
        self.count += 1;
        self.parameters_item.push(x.0);
        self.data_item.push(x.1);
    }

    pub(crate) fn encode(
        self,
        dst: &mut BytesMut,
    ) {
        dst.put_u8(self.count);
        self.parameters_item
            .into_iter()
            .for_each(|x| x.encode(dst));
        self.data_item
            .into_iter()
            .for_each(|x| x.encode(dst));
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct WriteVarAckData {
    count: u8,
    data_item: Vec<DataItemWriteResponse>,
}

impl WriteVarAckData {
    pub fn data_item(
        self,
    ) -> Vec<DataItemWriteResponse> {
        self.data_item
    }

    pub fn add_response(
        mut self,
        response: DataItemWriteResponse,
    ) -> Self {
        self.count += 1;
        self.data_item.push(response);
        self
    }

    pub(crate) fn encode(
        self,
        dst: &mut BytesMut,
    ) {
        dst.put_u8(self.count);
        self.data_item
            .into_iter()
            .for_each(|x| x.encode(dst));
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct ReadVarJob {
    count: u8,
    parameters_item: Vec<ItemRequest>,
}
impl ReadVarJob {
    pub fn bytes_len_data(&self) -> u16 {
        0
    }

    pub fn bytes_len_parameter(&self) -> u16 {
        self.parameters_item
            .iter()
            .fold(2, |len, x| len + x.bytes_len())
    }

    pub fn add_item(&mut self, x: ItemRequest) {
        self.count += 1;
        self.parameters_item.push(x);
    }

    pub(crate) fn encode(
        self,
        dst: &mut BytesMut,
    ) {
        dst.put_u8(self.count);
        self.parameters_item
            .into_iter()
            .for_each(|x| x.encode(dst));
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct ReadVarAckData {
    count: u8,
    data_item: Vec<DataItemVal>,
}
impl ReadVarAckData {
    pub fn data_item(self) -> Vec<DataItemVal> {
        self.data_item
    }

    pub fn add_response(
        mut self,
        value: DataItemVal,
    ) -> Self {
        self.count += 1;
        self.data_item.push(value);
        self
    }

    pub(crate) fn encode(
        self,
        dst: &mut BytesMut,
    ) {
        dst.put_u8(self.count);
        self.data_item
            .into_iter()
            .for_each(|x| x.encode(dst));
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct SetupCommunication {
    reserved: u8,
    max_amq_calling: u16,
    max_amq_called: u16,
    pdu_length: u16,
}

impl SetupCommunication {
    pub fn init(
        max_amq_calling: u16,
        max_amq_called: u16,
        pdu_length: u16,
    ) -> Self {
        Self {
            reserved: 0,
            max_amq_calling,
            max_amq_called,
            pdu_length,
        }
    }

    fn len() -> usize {
        7
    }

    pub(crate) fn encode(
        self,
        dst: &mut BytesMut,
    ) {
        dst.put_u8(self.reserved);
        dst.extend_from_slice(
            self.max_amq_calling
                .to_be_bytes()
                .as_slice(),
        );
        dst.extend_from_slice(
            self.max_amq_called
                .to_be_bytes()
                .as_slice(),
        );
        dst.extend_from_slice(
            self.pdu_length
                .to_be_bytes()
                .as_slice(),
        );
    }

    fn decode(
        src: &mut BytesMut,
    ) -> Result<Self> {
        if src.len() < Self::len() {
            return Err(Error::Error(
                "data of SetupCommunication not \
                 enough"
                    .to_string(),
            ));
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

    pub fn pdu_length(&self) -> u16 {
        self.pdu_length
    }
}

const PARAM_ITEM_VAR_SPEC: u8 = 0x12;
const PARAM_ITEM_VAR_SPEC_LENGTH: u8 = 0x0a;

#[derive(Debug, Eq, PartialEq)]
pub struct ItemRequest {
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
    pub fn new(
        transport_size_type: TransportSize,
        db_number: DbNumber,
        area: Area,
        byte_addr: u16,
        bit_addr: u8,
        length: u16,
    ) -> Self {
        Self {
            variable_specification:
                PARAM_ITEM_VAR_SPEC,
            follow_length:
                PARAM_ITEM_VAR_SPEC_LENGTH,
            syntax_id: Syntax::S7Any,
            transport_size_type,
            length,
            db_number,
            area,
            address: Address {
                byte_addr,
                bit_addr,
            },
        }
    }

    /*
    pub fn init_db_byte(
        db_number: u16,
        byte_addr: u16,
        bit_addr: u8,
        length: u16,
    ) -> Self {
        Self {
            variable_specification:
                PARAM_ITEM_VAR_SPEC,
            follow_length:
                PARAM_ITEM_VAR_SPEC_LENGTH,
            syntax_id: Syntax::S7Any,
            transport_size_type:
                TransportSize::NoBit,
            length,
            db_number: DbNumber::DbNumber(
                db_number,
            ),
            area: Area::DataBlocks,
            address: Address {
                byte_addr,
                bit_addr,
            },
        }
    }

    pub fn init_db_bit(
        db_number: u16,
        byte_addr: u16,
        bit_addr: u8,
    ) -> Self {
        Self {
            variable_specification:
                PARAM_ITEM_VAR_SPEC,
            follow_length:
                PARAM_ITEM_VAR_SPEC_LENGTH,
            syntax_id: Syntax::S7Any,
            transport_size_type:
                TransportSize::Bit,
            length: 1,
            db_number: DbNumber::DbNumber(
                db_number,
            ),
            area: Area::DataBlocks,
            address: Address {
                byte_addr,
                bit_addr,
            },
        }
    }
    */

    pub fn init_byte(
        db_number: Option<u16>,
        area: Area,
        byte_addr: u16,
        length: u16,
    ) -> Self {
        let db_number = match db_number {
            Some(x) => DbNumber::DbNumber(x),
            None => DbNumber::NotIn,
        };

        Self {
            variable_specification:
                PARAM_ITEM_VAR_SPEC,
            follow_length:
                PARAM_ITEM_VAR_SPEC_LENGTH,
            syntax_id: Syntax::S7Any,
            transport_size_type:
                TransportSize::NoBit,
            length,
            db_number,
            area,
            address: Address {
                byte_addr,
                bit_addr: 0,
            },
        }
    }

    pub fn init_bit(
        db_number: Option<u16>,
        area: Area,
        byte_addr: u16,
        bit_addr: u8,
    ) -> Self {
        let db_number = match db_number {
            Some(x) => DbNumber::DbNumber(x),
            None => DbNumber::NotIn,
        };

        Self {
            variable_specification:
                PARAM_ITEM_VAR_SPEC,
            follow_length:
                PARAM_ITEM_VAR_SPEC_LENGTH,
            syntax_id: Syntax::S7Any,
            transport_size_type:
                TransportSize::Bit,
            length: 1,
            db_number,
            area,
            address: Address {
                byte_addr,
                bit_addr,
            },
        }
    }

    pub fn bytes_len(&self) -> u16 {
        12
    }

    fn encode(self, dst: &mut BytesMut) {
        dst.put_u8(self.variable_specification);
        dst.put_u8(self.follow_length);
        dst.put_u8(self.syntax_id.into());
        dst.put_u8(
            self.transport_size_type.into(),
        );
        dst.extend_from_slice(
            self.length.to_be_bytes().as_slice(),
        );
        dst.put_u16(self.db_number.into());
        dst.put_u8(self.area.into());
        dst.extend_from_slice(
            self.address.to_bytes().as_slice(),
        );
    }

    fn decode(
        src: &mut BytesMut,
    ) -> Result<Self> {
        if src.len() < 12 {
            return Err(Error::Error(
                "item request byte's length is not enough"
                    .to_string(),
            ));
        }
        let variable_specification = src.get_u8();
        let follow_length = src.get_u8();
        let syntax_id =
            Syntax::from(src.get_u8());
        let transport_size_type =
            TransportSize::from(src.get_u8());
        let length = src.get_u16();
        let db_number =
            DbNumber::from(src.get_u16());
        let area = Area::from(src.get_u8());
        let address = Address::from_bytes(
            src.get_u8(),
            src.get_u8(),
            src.get_u8(),
        );
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DataItemWriteResponse {
    pub return_code: ReturnCode,
}

impl DataItemWriteResponse {
    pub fn init(return_code: ReturnCode) -> Self {
        Self { return_code }
    }

    fn encode(self, dst: &mut BytesMut) {
        dst.put_u8(self.return_code.into());
    }

    fn decode(
        src: &mut BytesMut,
    ) -> Result<Self> {
        if src.len() == 0 {
            return Err(Error::Error(
                "byte's length is zero"
                    .to_string(),
            ));
        }
        Ok(Self {
            return_code: ReturnCode::try_from(
                src.get_u8(),
            )?,
        })
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DataItemVal {
    pub return_code: ReturnCode,
    pub transport_size_type: DataTransportSize,
    // 位查询,返回长度为0x0001; 非位查询,长度须左移3位
    pub length: u16,
    pub data: Vec<u8>,
}

impl DataItemVal {
    pub fn init_with_bytes(
        return_code: ReturnCode,
        data: &[u8],
    ) -> Self {
        Self {
            return_code,
            transport_size_type:
                DataTransportSize::NoBit,
            length: (data.len() as u16) << 3,
            data: data.to_vec(),
        }
    }

    pub fn init_with_bit(
        return_code: ReturnCode,
        data: bool,
    ) -> Self {
        Self {
            return_code,
            transport_size_type:
                DataTransportSize::Bit,
            length: 1,
            data: if data {
                vec![1]
            } else {
                vec![0]
            },
        }
    }

    pub fn bytes_len(&self) -> u16 {
        self.data.len() as u16 + 4
    }

    fn encode(self, dst: &mut BytesMut) {
        dst.put_u8(self.return_code.into());
        dst.put_u8(
            self.transport_size_type.into(),
        );
        dst.extend_from_slice(
            self.length.to_be_bytes().as_slice(),
        );
        dst.extend_from_slice(
            self.data.as_slice(),
        );
    }

    fn decode(
        src: &mut BytesMut,
    ) -> Result<Self> {
        if src.len() < 4 {
            return Err(Error::Error(
                format!("data item val byte's length is not enough: {}"
                    , src.len()),
            ));
        }
        let return_code =
            ReturnCode::try_from(src.get_u8())?;
        let transport_size_type =
            DataTransportSize::from(src.get_u8());
        let length = src.get_u16();
        let mut bytes_len = length as usize;

        if transport_size_type
            == DataTransportSize::NoBit
        {
            bytes_len >>= 3;
        }

        let fill_byte_len = bytes_len % 2;
        if src.len() < bytes_len {
            return Err(Error::Error(
                format!("data item val byte's length is not enough: {} < {}", src.len(), bytes_len),
            ));
        }
        let mut data =
            Vec::with_capacity(bytes_len);
        for _ in 0..bytes_len {
            data.push(src.get_u8())
        }
        if fill_byte_len > 0 && src.len() >= 1 {
            src.get_u8();
        }
        Ok(Self {
            return_code,
            transport_size_type,
            length,
            data,
        })
    }
}

#[derive(
    Debug,
    Clone,
    IntoPrimitive,
    TryFromPrimitive,
    Eq,
    PartialEq,
)]
#[repr(u8)]
pub enum ReturnCode {
    /// 0
    Reserved = 0,
    /// Hardware error
    HwFault = 1,
    /// Accessing the object not allowed
    NotAllow = 3,
    /// Invalid address
    InvalidAddress = 5,
    /// Data type not supported
    NotSupported = 6,
    /// Data type inconsistent
    SizeMismatch = 7,
    /// Object does not exist
    Err = 0x0a,
    /// Success
    Success = 0xff,
}

impl ReturnCode {
    pub fn is_ok(&self) -> bool {
        *self == ReturnCode::Success
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    IntoPrimitive,
    Eq,
    FromPrimitive,
    PartialEq,
)]
#[repr(u8)]
pub enum TransportType {
    Bit = 0,
    Byte = 1,
    Word = 2,
    DWord = 3,
    Float = 4,
    #[num_enum(catch_all)]
    NotSupport(u8),
}

#[derive(
    Debug,
    Copy,
    Clone,
    IntoPrimitive,
    Eq,
    FromPrimitive,
    PartialEq,
)]
#[repr(u8)]
pub enum DataTransportSize {
    Bit = 0x03,
    NoBit = 0x04,
    #[num_enum(catch_all)]
    NotSupport(u8),
}

#[derive(
    Debug,
    Copy,
    Clone,
    IntoPrimitive,
    Eq,
    FromPrimitive,
    PartialEq,
)]
#[repr(u8)]
pub enum TransportSize {
    Bit = 0x01,
    NoBit = 0x02,
    #[num_enum(catch_all)]
    NotSupport(u8),
}

#[derive(
    Debug,
    IntoPrimitive,
    FromPrimitive,
    Eq,
    PartialEq,
)]
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

#[derive(
    Debug,
    IntoPrimitive,
    FromPrimitive,
    Eq,
    PartialEq,
)]
#[repr(u8)]
pub enum Syntax {
    S7Any = 0x10,
    #[num_enum(catch_all)]
    NotSupport(u8),
}
#[derive(
    Debug,
    IntoPrimitive,
    FromPrimitive,
    Eq,
    PartialEq,
)]
#[repr(u16)]
pub enum DbNumber {
    NotIn = 0,
    #[num_enum(catch_all)]
    DbNumber(u16),
}
#[derive(Debug, Eq, PartialEq)]
pub struct Address {
    byte_addr: u16,
    bit_addr: u8,
}

impl Address {
    pub fn to_bytes(&self) -> [u8; 3] {
        let [byte_0, byte_1] =
            self.byte_addr.to_be_bytes();
        [
            byte_0 >> 5,
            byte_0 << 3 | byte_1 >> 5,
            byte_1 << 3 | self.bit_addr,
        ]
    }

    pub fn from_bytes(
        index_0: u8,
        index_1: u8,
        index_2: u8,
    ) -> Self {
        let index_0 = index_0 << 5 | index_1 >> 3;
        let index_1 = index_1 << 5 | index_2 >> 3;

        let byte_addr = u16::from_be_bytes([
            index_0, index_1,
        ]);
        let bit_addr = index_2 & 0b0000_0111;
        Self {
            byte_addr,
            bit_addr,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Address;

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

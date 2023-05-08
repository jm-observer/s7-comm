use bytes::BytesMut;
use copt::{
    packet::{CoptFrame, DtData, PduType},
    CoptDecoder,
};
use s7_comm::{
    AckData, DataItemVal, DataItemWriteResponse, Frame, HearderAckData, ReadVarAckData, ReturnCode,
    S7CommDecoder, SetupCommunication, WriteVarAckData,
};
use tokio_util::codec::Decoder;

#[test]
fn s7_comm_ack_data_setup_decode() {
    let bytes: [u8; 23] = [
        0x02, 0xf0, 0x80, 0x32, 0x03, 0x00, 0x00, 0x04, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00,
        0xf0, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0xf0,
    ];
    let mut src = BytesMut::from(bytes.as_ref());
    let mut decoder = CoptDecoder(S7CommDecoder);
    let frame_builder = decoder.decode(&mut src);
    assert!(frame_builder.is_ok());
    if let Ok(res) = frame_builder {
        assert!(res.is_some());
        if let Some(res) = res {
            let header = HearderAckData::init(1024, 8, 0, 0, 0);
            let setup = SetupCommunication::init(1, 1, 240);
            let f = Frame::AckData {
                header,
                ack_data: AckData::SetupCommunication(setup),
            };
            assert_eq!(
                res,
                CoptFrame {
                    pdu_type: PduType::DtData(DtData {
                        tpdu_number: 0,
                        last_data_unit: true,
                        payload: f,
                    }),
                }
            );
        }
    }
}

#[test]
fn s7_comm_ack_data_write_var_decode() {
    let bytes: [u8; 18] = [
        0x02, 0xf0, 0x80, 0x32, 0x03, 0x00, 0x00, 0x05, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00,
        0x05, 0x01, 0xff,
    ];
    let mut src = BytesMut::from(bytes.as_ref());
    let mut decoder = CoptDecoder(S7CommDecoder);
    let frame_builder = decoder.decode(&mut src);
    assert!(frame_builder.is_ok());
    if let Ok(res) = frame_builder {
        assert!(res.is_some());
        if let Some(res) = res {
            let header = HearderAckData::init(1280, 2, 1, 0, 0);
            let ack = WriteVarAckData::default()
                .add_response(DataItemWriteResponse::init(ReturnCode::Success));

            let f = Frame::AckData {
                header,
                ack_data: AckData::WriteVar(ack),
            };
            assert_eq!(
                res,
                CoptFrame {
                    pdu_type: PduType::DtData(DtData {
                        tpdu_number: 0,
                        last_data_unit: true,
                        payload: f,
                    }),
                }
            );
        }
    }
}

#[test]
fn s7_comm_ack_data_read_var_decode() {
    let bytes: [u8; 25] = [
        0x02, 0xf0, 0x80, 0x32, 0x03, 0x00, 0x00, 0x05, 0x00, 0x00, 0x02, 0x00, 0x08, 0x00, 0x00,
        0x04, 0x01, 0xff, 0x04, 0x00, 0x20, 0x00, 0x00, 0x00, 0x79,
    ];
    let mut src = BytesMut::from(bytes.as_ref());
    let mut decoder = CoptDecoder(S7CommDecoder);
    let frame_builder = decoder.decode(&mut src);
    assert!(frame_builder.is_ok());
    if let Ok(res) = frame_builder {
        assert!(res.is_some());
        if let Some(res) = res {
            let header = HearderAckData::init(1280, 2, 8, 0, 0);
            let ack = ReadVarAckData::default().add_response(DataItemVal::init_with_bytes(
                ReturnCode::Success,
                [0x00, 0x00, 0x00, 0x79].as_ref(),
            ));
            let f = Frame::AckData {
                header,
                ack_data: AckData::ReadVar(ack),
            };
            assert_eq!(
                res,
                CoptFrame {
                    pdu_type: PduType::DtData(DtData {
                        tpdu_number: 0,
                        last_data_unit: true,
                        payload: f,
                    }),
                }
            );
        }
    }
}

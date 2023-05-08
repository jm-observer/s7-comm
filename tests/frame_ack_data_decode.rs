use bytes::BytesMut;
use s7_comm::{
    AckData, DataItemVal, DataItemWriteResponse, Frame, HearderAckData, ReadVarAckData, ReturnCode,
    S7CommDecoder, SetupCommunication, WriteVarAckData,
};
use tokio_util::codec::Decoder;

#[test]
fn setup_decode() {
    let bytes: [u8; 20] = [
        0x32, 0x03, 0x00, 0x00, 0x04, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x00, 0x00,
        0x01, 0x00, 0x01, 0x00, 0xf0,
    ];
    let mut src = BytesMut::from(bytes.as_ref());
    let mut decoder = S7CommDecoder;
    let frame_builder = decoder.decode(&mut src);
    assert!(frame_builder.is_ok());
    if let Ok(res) = frame_builder {
        assert!(res.is_some());
        if let Some(res) = res {
            let Frame::AckData {
                header, ack_data
            } = res else {
                unreachable!()
            };

            let header_right = HearderAckData::init(1024, 8, 0, 0, 0);
            assert_eq!(header_right, header);

            let setup = SetupCommunication::init(1, 1, 240);
            assert_eq!(ack_data, AckData::SetupCommunication(setup));
        }
    }
}

#[test]
fn write_var_decode() {
    let bytes: [u8; 15] = [
        0x32, 0x03, 0x00, 0x00, 0x05, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x05, 0x01, 0xff,
    ];
    let mut src = BytesMut::from(bytes.as_ref());
    let mut decoder = S7CommDecoder;
    let frame_builder = decoder.decode(&mut src);
    assert!(frame_builder.is_ok());
    if let Ok(res) = frame_builder {
        assert!(res.is_some());
        if let Some(res) = res {
            let Frame::AckData {
                header, ack_data
            } = res else {
                unreachable!()
            };

            let header_right = HearderAckData::init(1280, 2, 1, 0, 0);
            assert_eq!(header_right, header);

            let ack = WriteVarAckData::default()
                .add_response(DataItemWriteResponse::init(ReturnCode::Success));
            assert_eq!(ack_data, AckData::WriteVar(ack));
        }
    }
}

#[test]
fn read_var_decode() {
    let bytes: [u8; 22] = [
        0x32, 0x03, 0x00, 0x00, 0x05, 0x00, 0x00, 0x02, 0x00, 0x08, 0x00, 0x00, 0x04, 0x01, 0xff,
        0x04, 0x00, 0x20, 0x00, 0x00, 0x00, 0x79,
    ];
    let mut src = BytesMut::from(bytes.as_ref());
    let mut decoder = S7CommDecoder;
    let frame_builder = decoder.decode(&mut src);
    assert!(frame_builder.is_ok());
    if let Ok(res) = frame_builder {
        assert!(res.is_some());
        if let Some(res) = res {
            let Frame::AckData {
                header, ack_data
            } = res else {
                unreachable!()
            };

            let header_right = HearderAckData::init(1280, 2, 8, 0, 0);
            assert_eq!(header_right, header);

            let ack = ReadVarAckData::default().add_response(DataItemVal::init_with_bytes(
                ReturnCode::Success,
                [0x00, 0x00, 0x00, 0x79].as_ref(),
            ));
            assert_eq!(ack_data, AckData::ReadVar(ack));
        }
    }
}

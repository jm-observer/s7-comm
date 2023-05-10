mod test_data;

use crate::test_data::test_copt_data::{
    init_copt_connect_confirm_frame, init_copt_connect_confirm_frame_bytes,
    init_copt_connect_request_frame, init_copt_connect_request_frame_bytes,
    init_copt_dt_data_frame, init_copt_dt_data_frame_bytes,
};
use bytes::BytesMut;
use copt::CoptDecoder;
use s7_comm::S7CommDecoder;
use tokio_util::codec::Decoder;

#[test]
fn test_dt_data_decode() {
    let mut decoder = CoptDecoder(S7CommDecoder);
    let mut src = BytesMut::new();
    src.extend_from_slice(init_copt_dt_data_frame_bytes());
    let rs = decoder.decode(&mut src);
    assert!(rs.is_ok());
    if let Ok(Some(frame)) = rs {
        let dst_frame = init_copt_dt_data_frame();
        assert_eq!(dst_frame, frame);
    } else {
        unreachable!()
    }
}

#[test]
fn test_connect_request_decode() {
    let mut decoder = CoptDecoder(S7CommDecoder);
    let mut src = BytesMut::new();
    src.extend_from_slice(init_copt_connect_request_frame_bytes());
    let rs = decoder.decode(&mut src);
    assert!(rs.is_ok());
    if let Ok(Some(frame)) = rs {
        let dst_frame = init_copt_connect_request_frame();
        assert_eq!(dst_frame, frame);
    } else {
        unreachable!()
    }
}

#[test]
fn test_connect_confirm_decode() {
    let mut decoder = CoptDecoder(S7CommDecoder);
    let mut src = BytesMut::new();
    src.extend_from_slice(init_copt_connect_confirm_frame_bytes());
    let rs = decoder.decode(&mut src);
    assert!(rs.is_ok());
    if let Ok(Some(frame)) = rs {
        let dst_frame = init_copt_connect_confirm_frame();
        assert_eq!(dst_frame, frame);
    } else {
        unreachable!()
    }
}

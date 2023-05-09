mod test_data;

use crate::test_data::test_copt_data::{init_copt_dt_data_frame, init_copt_dt_data_frame_bytes};
use bytes::BytesMut;
use copt::CoptEncoder;
use s7_comm::S7CommEncoder;
use tokio_util::codec::Encoder;

#[test]
fn test_connect_request_encode() {
    let frame = init_copt_dt_data_frame();
    let mut encoder = CoptEncoder(S7CommEncoder);
    let mut dst = BytesMut::new();
    let res = encoder.encode(frame, &mut dst);
    assert!(res.is_ok());
    assert_eq!(dst.as_ref(), init_copt_dt_data_frame_bytes());
}

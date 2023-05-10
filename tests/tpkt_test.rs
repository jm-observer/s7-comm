mod test_data;

use crate::test_data::test_tpkt_data::{init_tpkt_frame, init_tpkt_frame_bytes};
use bytes::BytesMut;
use copt::{CoptDecoder, CoptEncoder};
use s7_comm::{S7CommDecoder, S7CommEncoder};
use tokio_util::codec::{Decoder, Encoder};
use tpkt::{TpktDecoder, TpktEncoder};

#[test]
fn test_decode() {
    let mut decoder = TpktDecoder(CoptDecoder(S7CommDecoder));
    let mut src = BytesMut::new();
    src.extend_from_slice(init_tpkt_frame_bytes());
    let rs = decoder.decode(&mut src);
    assert!(rs.is_ok());
    if let Ok(Some(frame)) = rs {
        let dst_frame = init_tpkt_frame();
        assert_eq!(dst_frame, frame);
    } else {
        unreachable!()
    }
}

#[test]
fn test_encode() {
    let mut edcoder = TpktEncoder(CoptEncoder(S7CommEncoder));
    let src_frame = init_tpkt_frame();
    let mut dst = BytesMut::new();
    let rs = edcoder.encode(src_frame, &mut dst);
    assert!(rs.is_ok());
    assert_eq!(init_tpkt_frame_bytes(), dst.as_ref())
}

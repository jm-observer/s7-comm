use bytes::BytesMut;
use s7_comm::{Frame, S7CommEncoder};
use tokio_util::codec::Encoder;

#[test]
fn encode() {
    let bytes: [u8; 24] = [
        0x32, 0x01, 0x00, 0x00, 0x05, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x04, 0x01, 0x12, 0x0a, 0x10,
        0x02, 0x00, 0x04, 0x00, 0x01, 0x84, 0x00, 0x09, 0x60,
    ];
    let frame_builder = Frame::job_read_var(1280);
    let frame = frame_builder.read_db_bytes(1, 300, 4).build();
    let mut dst = BytesMut::new();
    let mut encoder = S7CommEncoder;
    assert!(encoder.encode(frame, &mut dst).is_ok());
    assert_eq!(dst.as_ref(), bytes.as_ref())
}

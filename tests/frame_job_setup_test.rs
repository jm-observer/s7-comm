use bytes::BytesMut;
use s7_comm::builder::FrameBuilder;
use s7_comm::S7CommEncoder;
use tokio_util::codec::Encoder;

#[test]
fn encode() {
    let bytes: [u8; 18] = [
        0x32, 0x01, 0x00, 0x00, 0x04, 0x00, 0x00, 0x08, 0x00, 0x00, 0xf0, 0x00, 0x00, 0x01, 0x00,
        0x01, 0x01, 0xe0,
    ];
    let frame_builder = FrameBuilder::job_setup(1024);
    let frame = frame_builder
        .max_amq_called(1)
        .max_amq_calling(1)
        .pdu_length(480)
        .build();
    let mut dst = BytesMut::new();
    let mut encoder = S7CommEncoder;
    assert!(encoder.encode(frame, &mut dst).is_ok());
    assert_eq!(dst.as_ref(), bytes.as_ref())
}
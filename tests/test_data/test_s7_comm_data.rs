use s7_comm::builder::FrameBuilder;
use s7_comm::Frame;

pub fn init_job_setup_frame() -> Frame {
    let frame_builder = FrameBuilder::job_setup(1024);
    frame_builder
        .max_amq_called(1)
        .max_amq_calling(1)
        .pdu_length(480)
        .build()
}

pub fn init_job_setup_frame_bytes() -> &'static [u8] {
    [
        0x32, 0x01, 0x00, 0x00, 0x04, 0x00, 0x00, 0x08, 0x00, 0x00, 0xf0, 0x00, 0x00, 0x01, 0x00,
        0x01, 0x01, 0xe0,
    ]
    .as_ref()
}

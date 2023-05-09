use crate::test_data::test_s7_comm_data::init_job_setup_frame;
use copt::CoptFrame;
use s7_comm::Frame;

pub fn init_copt_dt_data_frame() -> CoptFrame<Frame> {
    let s7_frame = init_job_setup_frame();
    CoptFrame::builder_of_dt_data(s7_frame).build(0, true)
}

pub fn init_copt_dt_data_frame_bytes() -> &'static [u8] {
    [
        0x02, 0xf0, 0x80, 0x32, 0x01, 0x00, 0x00, 0x04, 0x00, 0x00, 0x08, 0x00, 0x00, 0xf0, 0x00,
        0x00, 0x01, 0x00, 0x01, 0x01, 0xe0,
    ]
    .as_ref()
}

use crate::test_data::test_copt_data::init_copt_connect_request_frame;
use copt::CoptFrame;
use s7_comm::Frame;
use tpkt::TpktFrame;

pub fn init_tpkt_frame() -> TpktFrame<CoptFrame<Frame>> {
    let s7_frame = init_copt_connect_request_frame();
    TpktFrame::new(s7_frame)
}

pub fn init_tpkt_frame_bytes() -> &'static [u8] {
    [
        3u8, 0, 0, 0x16, 0x11, 0xe0, 0x00, 0x00, 0x00, 0x01, 0x00, 0xc0, 0x01, 0x0a, 0xc1, 0x02,
        0x01, 0x00, 0xc2, 0x02, 0x02, 0x01,
    ]
    .as_ref()
}

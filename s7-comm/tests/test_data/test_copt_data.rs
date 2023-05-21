use super::test_s7_comm_data::init_job_setup_frame;
use copt::{CoptFrame, Parameter, TpduSize};
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

pub fn init_copt_connect_request_frame() -> CoptFrame<Frame> {
    CoptFrame::<Frame>::builder_of_connect()
        .source_ref([0, 1])
        .destination_ref([0, 0])
        .class_and_others(0, false, false)
        .push_parameter(Parameter::new_tpdu_size(TpduSize::L1024))
        .push_parameter(Parameter::new_src_tsap([1u8, 0].to_vec()))
        .push_parameter(Parameter::new_dst_tsap([2u8, 1].to_vec()))
        .build_to_request()
}

pub fn init_copt_connect_request_frame_bytes() -> &'static [u8] {
    [
        0x11, 0xe0, 0x00, 0x00, 0x00, 0x01, 0x00, 0xc0, 0x01, 0x0a, 0xc1, 0x02, 0x01, 0x00, 0xc2,
        0x02, 0x02, 0x01,
    ]
    .as_ref()
}

pub fn init_copt_connect_confirm_frame() -> CoptFrame<Frame> {
    CoptFrame::<Frame>::builder_of_connect()
        .source_ref([0, 8])
        .destination_ref([0, 1])
        .class_and_others(0, false, false)
        .push_parameter(Parameter::new_tpdu_size(TpduSize::L1024))
        .push_parameter(Parameter::new_src_tsap([1u8, 0].to_vec()))
        .push_parameter(Parameter::new_dst_tsap([2u8, 1].to_vec()))
        .build_to_confirm()
}

pub fn init_copt_connect_confirm_frame_bytes() -> &'static [u8] {
    [
        0x11, 0xd0, 0x00, 0x01, 0x00, 0x08, 0x00, 0xc0, 0x01, 0x0a, 0xc1, 0x02, 0x01, 0x00, 0xc2,
        0x02, 0x02, 0x01,
    ]
    .as_ref()
}

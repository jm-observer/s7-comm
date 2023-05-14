use crate::codec::S7Encoder;
use crate::error::*;
use bytes::BytesMut;
use copt::{ConnectComm, CoptFrame, Parameter, PduType, TpduSize};
use tokio_util::codec::Encoder;
use tpkt::TpktFrame;

#[derive(Default)]
pub struct CoptSetupBuilder {
    pdu_ref: u16,
    max_amq_calling: u16,
    max_amq_called: u16,
    pdu_length: u16,
}

impl CoptSetupBuilder {
    pub fn pdu_ref(mut self, pdu_ref: u16) -> Self {
        self.pdu_ref = pdu_ref;
        self
    }
    pub fn max_amq_calling(mut self, max_amq_calling: u16) -> Self {
        self.max_amq_calling = max_amq_calling;
        self
    }
    pub fn max_amq_called(mut self, max_amq_called: u16) -> Self {
        self.max_amq_called = max_amq_called;
        self
    }
    pub fn pdu_length(mut self, pdu_length: u16) -> Self {
        self.pdu_length = pdu_length;
        self
    }

    pub fn build_to_request(self) -> Result<BytesMut> {
        let frame = TpktFrame::new(
            CoptFrame::builder_of_dt_data(
                s7_comm::Frame::job_setup(self.pdu_ref)
                    .pdu_length(self.pdu_length)
                    .max_amq_calling(self.max_amq_calling)
                    .max_amq_called(self.max_amq_called)
                    .build(),
            )
            .build(0, true),
        );
        let mut dst = BytesMut::new();
        let mut encoder = S7Encoder::default();
        encoder.encode(frame, &mut dst)?;
        Ok(dst)
    }
}

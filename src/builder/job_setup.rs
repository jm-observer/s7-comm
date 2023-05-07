use crate::packet::{Frame, Header, Job, SetupCommunication};

#[derive(Default)]
pub struct FrameJobSetupBuilder {
    pdu_ref: u16,
    max_amq_calling: u16,
    max_amq_called: u16,
    pdu_length: u16,
}

impl FrameJobSetupBuilder {
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
    pub fn build(self) -> Frame {
        let Self {
            pdu_ref,
            max_amq_calling,
            max_amq_called,
            pdu_length,
        } = self;
        let header = Header::init(pdu_ref, 8, 0);
        let setup = SetupCommunication::init(max_amq_calling, max_amq_called, pdu_length);
        let job = Job::SetupCommunication(setup);
        Frame::Job { header, job }
    }
}

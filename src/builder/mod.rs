mod job_setup;

use crate::builder::job_setup::FrameJobSetupBuilder;


pub struct FrameBuilder;

impl FrameBuilder {
    pub fn job_setup(pdu_ref: u16) -> FrameJobSetupBuilder {
        FrameJobSetupBuilder::default().pdu_ref(pdu_ref)
    }
}

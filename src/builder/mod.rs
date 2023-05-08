mod job_read_var;
mod job_setup;
mod job_write_var;

use crate::builder::job_read_var::FrameJobReadVarBuilder;
use crate::builder::job_setup::FrameJobSetupBuilder;
use crate::builder::job_write_var::FrameJobWriteVarBuilder;

pub struct FrameBuilder;

impl FrameBuilder {
    pub fn job_setup(pdu_ref: u16) -> FrameJobSetupBuilder {
        FrameJobSetupBuilder::default().pdu_ref(pdu_ref)
    }

    pub fn job_write_var(pdu_ref: u16) -> FrameJobWriteVarBuilder {
        FrameJobWriteVarBuilder::default().pdu_ref(pdu_ref)
    }

    pub fn job_read_var(pdu_ref: u16) -> FrameJobReadVarBuilder {
        FrameJobReadVarBuilder::default().pdu_ref(pdu_ref)
    }
}

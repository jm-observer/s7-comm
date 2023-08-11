use crate::packet::{
    Frame, Header, ItemRequest, Job, ReadVarJob,
};
use crate::Area;

#[derive(Default)]
pub struct FrameJobReadVarBuilder {
    pdu_ref: u16,
    items: Vec<ItemRequest>,
}

impl FrameJobReadVarBuilder {
    pub fn pdu_ref(
        mut self,
        pdu_ref: u16,
    ) -> Self {
        self.pdu_ref = pdu_ref;
        self
    }

    pub fn add_item(
        mut self,
        item: ItemRequest,
    ) -> Self {
        self.items.push(item);
        self
    }

    pub fn read_bytes(
        self,
        db_number: Option<u16>,
        area: Area,
        byte_addr: u16,
        len: u16,
    ) -> Self {
        let req = ItemRequest::init_byte(
            db_number, area, byte_addr, len,
        );
        self.add_item(req)
    }

    pub fn build(self) -> Frame {
        let Self { pdu_ref, items } = self;

        let job = items.into_iter().fold(
            ReadVarJob::default(),
            |mut job, item| {
                job.add_item(item);
                job
            },
        );

        let data_len = job.bytes_len_data();
        let parameter_len =
            job.bytes_len_parameter();
        let header = Header::init(
            pdu_ref,
            parameter_len,
            data_len,
        );

        let job = Job::ReadVar(job);

        Frame::Job { header, job }
    }
}

use crate::packet::{
    DataItemVal, Frame, Header, ItemRequest, Job, WriteVarJob,
};

#[derive(Default)]
pub struct FrameJobWriteVarBuilder {
    pdu_ref: u16,
    items: Vec<(ItemRequest, DataItemVal)>,
}

impl FrameJobWriteVarBuilder {
    pub fn pdu_ref(mut self, pdu_ref: u16) -> Self {
        self.pdu_ref = pdu_ref;
        self
    }
    pub fn add_item(mut self, item: (ItemRequest, DataItemVal)) -> Self {
        self.items.push(item);
        self
    }

    pub fn write_db_bytes(
        self,
        db_number: u16,
        byte_addr: u16,
        bit_addr: u8,
        data: &[u8],
    ) -> Self {
        let req = ItemRequest::init_db_byte(db_number, byte_addr, bit_addr, data.len() as u16);
        let data_val = DataItemVal::init_with_bytes(data);
        self.add_item((req, data_val))
    }

    pub fn build(self) -> Frame {
        let Self { pdu_ref, items } = self;

        let job = items
            .into_iter()
            .fold(WriteVarJob::default(), |mut job, item| {
                job.add_item(item);
                job
            });

        let data_len = job.bytes_len_data();
        let parameter_len = job.bytes_len_parameter();
        let header = Header::init(pdu_ref, parameter_len, data_len);

        let job = Job::WriteVar(job);

        Frame::Job { header, job }
    }
}

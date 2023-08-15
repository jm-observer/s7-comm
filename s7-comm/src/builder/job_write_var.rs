use crate::packet::{
    DataItemVal, Frame, Header, ItemRequest, Job,
    ReturnCode, WriteVarJob,
};
use crate::Area;

#[derive(Default)]
pub struct FrameJobWriteVarBuilder {
    pdu_ref: u16,
    items: Vec<(ItemRequest, DataItemVal)>,
}

impl FrameJobWriteVarBuilder {
    pub fn pdu_ref(
        mut self,
        pdu_ref: u16,
    ) -> Self {
        self.pdu_ref = pdu_ref;
        self
    }
    pub fn add_item(
        mut self,
        item: (ItemRequest, DataItemVal),
    ) -> Self {
        self.items.push(item);
        self
    }

    // todo 增加其他类型。应该也可以再抽象
    pub fn write_bytes(
        self,
        db_number: Option<u16>,
        area: Area,
        byte_addr: u16,
        data: &[u8],
    ) -> Self {
        let req = ItemRequest::init_byte(
            db_number,
            area,
            byte_addr,
            data.len() as u16,
        );
        let data_val =
            DataItemVal::init_with_bytes(
                ReturnCode::Reserved,
                data,
            );
        self.add_item((req, data_val))
    }

    pub fn build(self) -> Frame {
        let Self { pdu_ref, items } = self;

        let job = items.into_iter().fold(
            WriteVarJob::default(),
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

        let job = Job::WriteVar(job);

        Frame::Job { header, job }
    }
}

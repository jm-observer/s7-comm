use crate::{codec::S7Encoder, error::*};
use bytes::BytesMut;
use copt::CoptFrame;
use s7_comm::ItemRequest;
use tokio_util::codec::Encoder;
use tpkt::TpktFrame;

#[derive(Default)]
pub struct S7ReadBuilder {
    pdu_ref: u16,
    items:   Vec<ItemRequest>
}

impl S7ReadBuilder {
    pub fn pdu_ref(
        mut self,
        pdu_ref: u16
    ) -> Self {
        self.pdu_ref = pdu_ref;
        self
    }

    pub fn add_item(
        mut self,
        item: ItemRequest
    ) -> Self {
        self.items.push(item);
        self
    }

    pub fn read_db_bytes(
        self,
        db_number: u16,
        byte_addr: u16,
        len: u16
    ) -> Self {
        let req = ItemRequest::init_db_byte(
            db_number, byte_addr, 0, len
        );
        self.add_item(req)
    }

    pub fn build(self) -> Result<BytesMut> {
        let mut read_builder =
            s7_comm::Frame::job_read_var(
                self.pdu_ref
            );

        for item in self.items {
            read_builder =
                read_builder.add_item(item);
        }
        let frame = TpktFrame::new(
            CoptFrame::builder_of_dt_data(
                read_builder.build()
            )
            .build(0, true)
        );
        let mut dst = BytesMut::new();
        let mut encoder = S7Encoder::default();
        encoder.encode(frame, &mut dst)?;
        Ok(dst)
    }
}

use crate::packet::{CoptFrame, DtData, PduType};
use std::fmt::Debug;

pub struct DtDataBuilder<F> {
    payload: F,
}

impl<F: Debug + Eq + PartialEq> DtDataBuilder<F> {
    pub fn new(payload: F) -> Self {
        Self { payload }
    }
    pub fn build(self, tpdu_number: u8, last_data_unit: bool) -> CoptFrame<F> {
        CoptFrame {
            pdu_type: PduType::DtData(DtData {
                tpdu_number,
                last_data_unit,
                payload: self.payload,
            }),
        }
    }
}

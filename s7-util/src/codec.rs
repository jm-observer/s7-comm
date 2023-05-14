use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct S7Encoder(tpkt::TpktEncoder<copt::CoptEncoder<s7_comm::S7CommEncoder>>);

impl Deref for S7Encoder {
    type Target = tpkt::TpktEncoder<copt::CoptEncoder<s7_comm::S7CommEncoder>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for S7Encoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

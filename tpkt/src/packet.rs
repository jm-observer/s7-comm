use crate::{ToTpktError, TpktEncoder};
use bytes::BytesMut;
use tokio_util::codec::Encoder;

#[derive(Debug, Eq, PartialEq)]
pub struct TpktFrame<F> {
    pub(crate) version: u8,
    pub(crate) payload: F,
}

impl<F> TpktFrame<F> {
    pub fn new(payload: F) -> Self {
        Self {
            version: 3,
            payload,
        }
    }
    pub fn version_mut(&mut self, version: u8) {
        self.version = version;
    }

    pub fn payload(self) -> F {
        self.payload
    }

    pub fn to_bytes<E>(self) -> Result<BytesMut, crate::error::Error>
    where
        E: Encoder<F> + Default,
        <E as Encoder<F>>::Error: ToTpktError + Send + Sync + 'static,
    {
        let mut encoder = TpktEncoder(E::default());
        let mut dst = BytesMut::new();
        encoder.encode(self, &mut dst)?;
        Ok(dst)
    }
}

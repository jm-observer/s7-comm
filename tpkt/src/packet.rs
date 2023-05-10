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
}

pub struct TpktFrame<F> {
    pub version: u8,
    pub reserved: u8,
    pub length: u16,
    pub payload: F,
}

use tokio_util::codec::{Decoder, Encoder};
use bytes::{Buf, BufMut, BytesMut};
use anyhow::{bail, Error};

pub struct TpktEncoder<E>(E);
pub struct TpktDecoder<D>(D);


pub struct TpktFrame<F> {
    pub version: u8,
    pub reserved: u8,
    pub payload: F,
}

impl <F, E: Encoder<F>> Encoder<TpktFrame<F>> for TpktEncoder<E> where <E as Encoder<F>>::Error: std::error::Error + Send + Sync + 'static {
    type Error = Error;

    fn encode(&mut self, item: TpktFrame<F>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let mut payload = BytesMut::new();
        self.0.encode(item.payload, &mut payload)?;
        let length = payload.len() as u16 + 4;
        dst.put_u8(item.version);
        dst.put_u8(item.reserved);
        dst.put_u16(length);
        dst.extend_from_slice(payload.as_ref());
        Ok(())
    }
}

impl <F, D: Decoder<Item=F>> Decoder for TpktDecoder<D> where <D as Decoder>::Error: std::error::Error + Send + Sync + 'static  {
    type Item = TpktFrame<F>;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
           return Ok(None)
        }
        let (Some(index_0), Some(index_1)) = (src.get(1), src.get(2)) else {
            unreachable!()
        };
        let length = u16::from_be_bytes([*index_0, *index_1]) as usize;
        if src.len() < length {
            return Ok(None);
        }
        let mut framed_datas = src.split_to(length);
        let version = framed_datas.get_u8();
        let reserved = framed_datas.get_u8();
        let _ = framed_datas.get_u16();
        let Some(payload) = self.0.decode(&mut framed_datas)? else {
            // maybe return none
            bail!("payload decode fail!");
        };
        Ok(Some(TpktFrame {
            version, reserved, payload
        }))
    }
}
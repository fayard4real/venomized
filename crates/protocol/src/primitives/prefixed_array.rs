use crate::error::ProtocolError;
use crate::{codec::Codec, primitives::varint::VarInt};
pub struct PrefixedArray<T> {
    pub length: VarInt,
    pub data: Vec<T>,
}

impl<T: Codec> Codec for PrefixedArray<T> {
    fn decode(reader: &mut impl buffer::Buffer) -> Result<Self, ProtocolError> {
        let length = VarInt::decode(reader)?;
        let len_usize = length.0 as usize;

        let mut data: Vec<T> = Vec::with_capacity(len_usize);

        for _ in 0..len_usize {
            let item = T::decode(reader)?;
            data.push(item);
        }

        Ok(Self { length, data })
    }

    fn encode(&self, writer: &mut impl buffer::BufferMut) -> Result<(), ProtocolError> {
        VarInt(self.data.len() as i32).encode(writer)?;

        for item in &self.data {
            item.encode(writer)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefixed_array_roundtrip() -> Result<(), ProtocolError> {
        let mut buf: Vec<u8> = Vec::new();
        VarInt(3).encode(&mut buf)?;
        VarInt(322).encode(&mut buf)?;
        VarInt(228).encode(&mut buf)?;
        VarInt(123).encode(&mut buf)?;

        let mut reader = &buf[..];

        let pr_ar: PrefixedArray<VarInt> = PrefixedArray::decode(&mut reader)?;

        assert_eq!(pr_ar.length.0, 3);
        assert_eq!(pr_ar.data[0].0, 322);
        assert_eq!(pr_ar.data[1].0, 228);
        assert_eq!(pr_ar.data[2].0, 123);
        Ok(())
    }
}

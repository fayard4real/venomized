use crate::{codec::Codec, primitives::varint::VarInt};

pub struct PrefixedArray<T> {
    pub length: VarInt,
    pub data: Vec<T>,
}

impl<T: Codec> Codec for PrefixedArray<T> {
    fn decode(reader: &mut impl buffer::Buffer) -> Result<Self, crate::error::ProtocolError> {
        let length = VarInt::decode(reader)?;
        let len_usize = length.0 as usize;

        let mut data: Vec<T> = Vec::with_capacity(len_usize);

        for _ in 0..len_usize {
            let item = T::decode(reader)?;
            data.push(item);
        }

        Ok(Self {
            length,
            data,
        })
    }

    fn encode(&self, writer: &mut impl buffer::BufferMut) -> Result<(), crate::error::ProtocolError> {
        VarInt(self.data.len() as i32).encode(writer)?;

        for item in &self.data {
            item.encode(writer)?;
        }

        Ok(())
    }
}
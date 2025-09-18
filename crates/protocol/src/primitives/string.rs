use crate::{codec::Codec, error::ProtocolError, primitives::varint::VarInt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringProto(pub String);

impl Codec for StringProto {
    fn decode(reader: &mut impl buffer::Buffer) -> Result<Self, ProtocolError> {
        let length = VarInt::decode(reader)?;
        let len_usize = length.0 as usize;
        let mut buf = vec![0u8; len_usize];
        reader.read_exact(&mut buf)?;
        let data = String::from_utf8(buf)?;

        Ok(StringProto(data))
    }
    fn encode(&self, writer: &mut impl buffer::BufferMut) -> Result<(), crate::error::ProtocolError> {
        let string_bytes = self.0.as_bytes();
        let length = string_bytes.len();

        VarInt::from(length as i32).encode(writer)?;
        writer.extend_from_slice(string_bytes)?;

        Ok(())
    }
}
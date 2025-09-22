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
    fn encode(
        &self,
        writer: &mut impl buffer::BufferMut,
    ) -> Result<(), crate::error::ProtocolError> {
        let string_bytes = self.0.as_bytes();
        let length = string_bytes.len();

        VarInt::from(length as i32).encode(writer)?;
        writer.write_all(string_bytes)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn stringproto_roundtrip() -> Result<(), ProtocolError> {
        // Mock binary string
        let mut stream: Vec<u8> = Vec::new();
        VarInt(11).encode(&mut stream)?;
        let str_test = "Hello world".to_string();
        stream.extend_from_slice(str_test.as_bytes());

        // our buffer
        let mut buf = &stream[..];
        let str = StringProto::decode(&mut buf)?;

        assert_eq!(str.0, str_test);
        let test_buf_test = &stream[..];
        let mut test_buf: Vec<u8> = Vec::new();
        str.encode(&mut test_buf)?;
        assert_eq!(test_buf, test_buf_test);

        Ok(())
    }
}

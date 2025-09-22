use crate::{codec::Codec, error::ProtocolError};

#[derive(Debug)]
pub struct Byte(pub u8);

impl Codec for Byte {
    fn decode(reader: &mut impl buffer::Buffer) -> Result<Self, ProtocolError> {
        Ok(Byte(reader.read_u8()?))
    }

    fn encode(&self, writer: &mut impl buffer::BufferMut) -> Result<(), ProtocolError> {
        writer.write_u8(self.0)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn byte_roundtrip() -> Result<(), ProtocolError> {
        let byte = Byte(255);
        let mut writer: Vec<u8> = Vec::new();
        byte.encode(&mut writer)?;
        let mut buf = &writer[..];
        let byte_trip = Byte::decode(&mut buf)?;
        assert_eq!(byte.0, byte_trip.0);
        Ok(())
    }

    #[test]
    fn byte_decode_io_fail() {
        let stream: Vec<u8> = Vec::new();
        let mut buf = &stream[..];
        let res = Byte::decode(&mut buf);
        assert_matches!(res, Err(ProtocolError::Io(_)))
    }
}

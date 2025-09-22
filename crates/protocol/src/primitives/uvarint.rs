use crate::{
    codec::Codec,
    error::{ProtocolError, ProtocolViolation},
    primitives::varint::VarInt,
};

/// unsigned variable integer = 32-bits unsigned integer
/// Defines the implementation of variable integer from protocol buffer
/// See more -> https://protobuf.dev/programming-guides/encoding/#varints
#[derive(Debug)]
pub struct UVarInt(pub u32);

impl Codec for UVarInt {
    fn decode(reader: &mut impl buffer::Buffer) -> Result<Self, ProtocolError> {
        let num = {
            let val = VarInt::decode(reader)?;
            if val.0 < 0 {
                return Err(ProtocolError::ProtocolViolation(
                    ProtocolViolation::NegativeUnsigned,
                ));
            } else {
                val
            }
        };
        // absolutely logic-safety after this check
        Ok(UVarInt(num.0 as u32))
    }

    fn encode(&self, writer: &mut impl buffer::BufferMut) -> Result<(), ProtocolError> {
        VarInt::encode(&VarInt(self.0 as i32), writer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn uvarint_roundtrip() -> Result<(), ProtocolError> {
        let uvarint = UVarInt(42);
        let mut reader: Vec<u8> = Vec::new();
        uvarint.encode(&mut reader)?;
        let mut buf = &reader[..];
        let num = UVarInt::decode(&mut buf)?;
        assert_eq!(num.0, uvarint.0);
        Ok(())
    }
    #[test]
    fn uvarint_decode_fail_protocol_violation() -> Result<(), ProtocolError> {
        let mut writer: Vec<u8> = Vec::new();
        VarInt(-58).encode(&mut writer)?;
        let mut buf = &writer[..];
        let res = UVarInt::decode(&mut buf);
        assert_matches!(
            res,
            Err(ProtocolError::ProtocolViolation(
                ProtocolViolation::NegativeUnsigned
            ))
        );
        Ok(())
    }
}

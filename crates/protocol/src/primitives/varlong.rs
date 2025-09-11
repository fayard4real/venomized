use std::ops::Deref;

use buffer::{Buffer, BufferMut};

use crate::{
    codec::Codec,
    error::{ProtocolError, ProtocolViolation},
};

const VARLONG_LENGTH: i8 = 10;

#[derive(Debug)]
pub struct VarLong(pub i64);

impl Deref for VarLong {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Codec for VarLong {
    fn decode(reader: &mut impl Buffer) -> Result<Self, ProtocolError> {
        let mut num_read = 0;
        let mut result: i64 = 0;
        loop {
            let byte = reader.read_u8()?;
            let value = (byte & 0b0111_1111) as i64;

            result |= value << (7 * num_read);

            if (byte & 0b1000_0000) == 0 {
                break;
            }

            num_read += 1;

            if num_read >= VARLONG_LENGTH {
                return Err(ProtocolError::ProtocolViolation(
                    ProtocolViolation::VarLongTooLong,
                ));
            }
            if (byte & 0b1000_0000) == 0 {
                break;
            }
        }
        Ok(VarLong(result))
    }

    fn encode(&self, writer: &mut impl BufferMut) -> Result<(), ProtocolError> {
        let mut value: u64 = self.0 as u64;
        loop {
            let mut byte = (value & 0b0111_1111) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0b1000_0000;
            }
            writer.write_u8(byte)?;
            if value == 0 {
                break;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn varint_decode_success() -> Result<(), ProtocolError> {
        let buf: [u8; 1] = [0x2C];
        let mut reader: &[u8] = &buf;
        let num = VarLong::decode(&mut reader)?;
        assert_eq!(44, num.0);
        Ok(())
    }

    #[test]
    fn varint_decode_fail_protocol_violation() {
        let buf: [u8; 11] = [
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
        ];
        let mut reader: &[u8] = &buf;
        let res = VarLong::decode(&mut reader);
        assert_matches!(
            res,
            Err(ProtocolError::ProtocolViolation(
                ProtocolViolation::VarLongTooLong
            ))
        );
    }

    #[test]
    fn varint_decode_fail_io() {
        let buf: [u8; 0] = [];
        let mut reader: &[u8] = &buf;
        let res = VarLong::decode(&mut reader);
        assert_matches!(res, Err(ProtocolError::Io(_)))
    }

    #[test]
    fn varint_roundtrip() -> Result<(), ProtocolError> {
        let comp = 44;

        let mut writer = Vec::new();
        VarLong(comp).encode(&mut writer)?;
        let mut writer_comp = &writer[..];
        let num_comp = VarLong::decode(&mut writer_comp)?;
        assert_eq!(comp, num_comp.0);
        Ok(())
    }
}

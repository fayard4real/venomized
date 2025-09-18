use buffer::{Buffer, BufferMut};

use crate::error::ProtocolError;

// TODO: Describe trait
pub trait Codec: Sized {
    fn encode(&self, writer: &mut impl BufferMut) -> Result<(), ProtocolError>;

    fn decode(reader: &mut impl Buffer) -> Result<Self, ProtocolError>;
}

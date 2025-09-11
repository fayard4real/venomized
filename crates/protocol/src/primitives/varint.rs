use std::ops::Deref;

use crate::codec::Codec;

const VARINT_LENGTH: i8 = 5;

pub struct VarInt(pub i32);

impl Deref for VarInt {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: implement
impl Codec for VarInt {
    fn decode() {
        
    }
    fn encode() {
        
    }
}
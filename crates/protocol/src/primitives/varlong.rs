use std::ops::Deref;

use crate::codec::Codec;

const VARLONG_LENGTH: i8 = 10;

pub struct VarLong(pub i64);

impl Deref for VarLong {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: implement
impl Codec for VarLong {
    fn decode() {
        
    }
    fn encode() {
        
    }
}
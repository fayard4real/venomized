pub mod error;

use std::io::{Read, Write};
pub trait Buffer: Read {
    fn read_u8(&mut self) -> Result<u8, std::io::Error> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_u32_be(&mut self) -> Result<u32, std::io::Error> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    fn read_i32_be(&mut self) -> Result<i32, std::io::Error> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }
}

pub trait BufferMut: Write {
    fn write_u8(&mut self, val: u8) -> Result<(), std::io::Error> {
        self.write_all(&[val])
    }

    fn write_i32_be(&mut self, val: i32) -> Result<(), std::io::Error> {
        self.write_all(&val.to_be_bytes())
    }
}

impl<R: Read + ?Sized> Buffer for R {}
impl<W: Write + ?Sized> BufferMut for W {}

use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum ProtocolViolation {
    VarIntTooLong,
    VarLongTooLong,
}

#[derive(Debug)]
pub enum ProtocolError {
    Io(std::io::Error),
    ProtocolViolation(ProtocolViolation),
    Utf8(FromUtf8Error)
}

impl From<std::io::Error> for ProtocolError {
    fn from(value: std::io::Error) -> Self {
        ProtocolError::Io(value)
    }
}

impl From<FromUtf8Error> for ProtocolError {
    fn from(value: FromUtf8Error) -> Self {
        ProtocolError::Utf8(value)
    }
}
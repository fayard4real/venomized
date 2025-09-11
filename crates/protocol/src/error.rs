// TODO: add errors

#[derive(Debug)]
pub enum ProtocolViolation {
    VarIntTooLong,
    VarLongTooLong,
}

#[derive(Debug)]
pub enum ProtocolError {
    Io(std::io::Error),
    ProtocolViolation(ProtocolViolation),
}

impl From<std::io::Error> for ProtocolError {
    fn from(value: std::io::Error) -> Self {
        ProtocolError::Io(value)
    }
}

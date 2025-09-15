//! All IDs that will be used by the server and client 
//! to exchange information are stored here.

pub enum HandshakeServerbound {
    Handshake
}

pub enum LoginServerbound {}

pub enum ConfigureServerbound {}

pub enum PlayServerbound {}

pub enum LoginClientbound {}

pub enum ConfigureClientbound {}

pub enum PlayClientbound {}

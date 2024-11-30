#[derive(Debug)]
pub enum Errors {
    InvalidField(String),
    InvalidPacket(String),
    IOError(std::io::Error),
    EncryptionError(openssl::error::ErrorStack),
    CompressionError(std::io::Error),
    NbtError(crab_nbt::error::Error),
}
pub type Result<T> = core::result::Result<T, Errors>;
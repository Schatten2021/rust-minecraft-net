use std::io::ErrorKind;

#[derive(Debug)]
pub enum Errors {
    InvalidField(String),
    InvalidEnum(String),
    InvalidPacket(String),
    IOError(std::io::Error),
    EncryptionError(openssl::error::ErrorStack),
    CompressionError(std::io::Error),
    UTF8Error(std::string::FromUtf8Error),
    NbtStringDecodeError(cesu8::Cesu8DecodingError),
}
pub type Result<T> = core::result::Result<T, Errors>;
impl From<cesu8::Cesu8DecodingError> for Errors {
    fn from(err: cesu8::Cesu8DecodingError) -> Self {
        Self::NbtStringDecodeError(err)
    }
}
impl From<std::string::FromUtf8Error> for Errors {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::UTF8Error(err)
    }
}
impl From<std::io::Error> for Errors {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            ErrorKind::InvalidInput => Errors::CompressionError(value),
            _ => Errors::IOError(value),
        }
    }
}
impl From<openssl::error::ErrorStack> for Errors {
    fn from(err: openssl::error::ErrorStack) -> Self {
        Self::EncryptionError(err)
    }
}
impl std::error::Error for Errors {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IOError(err) => Some(err),
            Self::NbtStringDecodeError(err) => Some(err),
            Self::UTF8Error(err) => Some(err),
            Self::CompressionError(err) => Some(err),
            Self::EncryptionError(err) => Some(err),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        match self {
            Errors::InvalidField(_) => "The field was invalid",
            Errors::InvalidEnum(_) => "There was an error decoding an enum variant",
            Errors::InvalidPacket(_) => "The packet was invalid",
            Errors::IOError(_) => "There was an error with the IO",
            Errors::EncryptionError(_) => "There was an error encrypting/decrypting data",
            Errors::CompressionError(_) => "There was an error (de)compressing data",
            Errors::UTF8Error(_) => "Some received String wasn't valid UTF-8",
            Errors::NbtStringDecodeError(_) => "Some received NBT string wasn't valid \"modified UTF-8\" (see Java)",
        }
    }
}
impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Errors::InvalidField(msg) => format!("Invalid field: {}", msg),
            Errors::InvalidEnum(msg) => format!("Invalid enum: {}", msg),
            Errors::InvalidPacket(msg) => format!("Invalid packet: {}", msg),
            Errors::IOError(e) => format!("IO Error: {}", e),
            Errors::EncryptionError(e) => format!("Encryption error: {}", e),
            Errors::CompressionError(e) => format!("Compression error: {}", e),
            Errors::UTF8Error(e) => format!("UTF8 error: {}", e),
            Errors::NbtStringDecodeError(e) => format!("NbtStringDecode error: {}", e),
        })
    }
}
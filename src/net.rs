use crate::errors::Errors;
use crate::fields::{encode_var_int, read_var_int_from_stream};
use crate::{Packet, PacketReader};
use flate2::read::{ZlibDecoder, ZlibEncoder};
use flate2::Compression;
use openssl::symm::{Cipher, Crypter, Mode};
use std::io::{Read, Write};
use std::net::TcpStream;


pub struct EncryptedTcp {
    stream: TcpStream,
    encryptor: Crypter,
    decryptor: Crypter,
}
impl EncryptedTcp {
    pub fn new(stream: TcpStream, secret: Vec<u8>) -> Result<Self, Errors> {
        Ok(Self {
            stream,
            encryptor: Crypter::new(Cipher::aes_128_cfb8(), Mode::Encrypt, &*secret, Some(&*secret)).map_err(|e| Errors::EncryptionError(e))?,
            decryptor: Crypter::new(Cipher::aes_128_cfb8(), Mode::Decrypt, &*secret, Some(&*secret)).map_err(|e| Errors::EncryptionError(e))?,
        })
    }
}
impl Read for EncryptedTcp {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut read_buff = vec![0; buf.len()];
        self.stream.read(&mut *read_buff)?;
        Ok(self.decryptor.update(&*read_buff, buf)?)
    }
}
impl Write for EncryptedTcp {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut encrypted_buff = vec![0; buf.len()];
        self.encryptor.update(buf, &mut *encrypted_buff)?;
        self.stream.write(&*encrypted_buff)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}

pub struct UnknownPacket {
    pub id: i32,
    pub reader: PacketReader,
}

pub fn send_packet<T: Packet>(packet: T, mut stream: impl Write, compression_threshold: Option<usize>) -> Result<(), Errors> {
    let mut data = encode_var_int(T::ID);
    data.append(&mut packet.to_bytes());
    let res = compress_packet_data(data, compression_threshold)?;
    stream.write_all(&*res).map_err(|e| Errors::IOError(e))
}

fn compress_packet_data(mut data: Vec<u8>, threshold: Option<usize>) -> Result<Vec<u8>, Errors> {
    #[cfg(not(debug_assertions))]
    let _data = data.clone();
    if threshold.is_none() {
        let mut res = encode_var_int(data.len() as i32);
        res.append(&mut data);
        return Ok(res);
    }

    if data.len() < threshold.unwrap() {
        let mut res = encode_var_int((data.len() + 1) as i32);
        res.append(&mut encode_var_int(0));
        res.append(&mut data);
        return Ok(res);
    }

    let mut data_length = encode_var_int(data.len() as i32);
    let mut encoder = ZlibEncoder::new(&*data, Compression::default());
    let mut compressed = Vec::new();
    if let Err(_e) = encoder.read_to_end(&mut compressed) {
        return Err(Errors::InvalidPacket("Invalid compression".to_string()));
    }
    let mut res = encode_var_int((compressed.len() + data_length.len()) as i32);
    res.append(&mut data_length);
    res.append(&mut compressed);
    Ok(res)
}

pub fn receive_packet<T: Packet>(mut stream: impl Read, with_compression: bool) -> Result<T, Errors> {
    let len = read_var_int_from_stream(&mut stream)? as usize;
    let mut data = vec![0;len];
    stream.read_exact(&mut *data).map_err(|e| Errors::IOError(e))?;
    let mut reader = decompress_packet_data(data, with_compression)?;
    let _id = reader.read_var_int()?;
    let packet = T::from_reader(&mut reader)?;
    if reader.len() != 0 {
        return Err(Errors::InvalidPacket("Packet not read to end. Invalid length".into()))
    }
    Ok(packet)
}
pub fn receive_unknown_packet(mut stream: impl Read, with_compression: bool) -> Result<UnknownPacket, Errors> {
    let len = read_var_int_from_stream(&mut stream)? as usize;
    let mut data = vec![0;len];
    stream.read(&mut *data).map_err(|e| Errors::IOError(e))?;
    let mut reader = decompress_packet_data(data, with_compression)?;
    let id = reader.read_var_int()?;
    if id == 0 {
        print!("");
    }
    Ok(UnknownPacket { id, reader })
}
fn decompress_packet_data(data: Vec<u8>, with_compression: bool) -> Result<PacketReader, Errors> {
    if data.len() == 0 {
        return Err(Errors::InvalidPacket("received packet with length 0".into()));
    }
    let mut reader = PacketReader::new(data);
    if !with_compression {
        return Ok(reader)
    }
    
    let uncompressed_length = reader.read_var_int()?;
    
    if uncompressed_length == 0 {
        return Ok(reader)
    }
    
    let remaining = reader.read_rest();
    
    let mut decoder = ZlibDecoder::new(&*remaining);
    let mut decompressed = Vec::new();
    match decoder.read_to_end(&mut decompressed) {
        Err(e) => Err(Errors::CompressionError(e)),
        Ok(count) => if count == uncompressed_length as usize {
            Ok(())
        } else {
            Err(Errors::InvalidPacket("decompressed size not equal to asserted size.".into()))
        }
    }?;
    Ok(PacketReader::new(decompressed))
}
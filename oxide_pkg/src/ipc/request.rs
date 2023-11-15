use std::io::{self, Read, Write};
use byteorder::{NetworkEndian, WriteBytesExt, ReadBytesExt};
use crate::ipc::utils::{DeserializePacket, extract_string, SerializePacket};

pub enum LocalRequest {
    SyncProject { root_dir: String },
}

impl From<&LocalRequest> for u8 {
    fn from(req: &LocalRequest) -> Self {
        match req {
            LocalRequest::SyncProject { .. } => 1,
        }
    }
}

impl SerializePacket for LocalRequest {
    /// Serialize Request to bytes (to send to server)
    fn serialize(&self, buf: &mut impl Write) -> io::Result<usize> {
        buf.write_u8(self.into())?; // Message Type byte
        let mut bytes_written: usize = 1;
        match self {
            LocalRequest::SyncProject { root_dir } => {
                // Write the variable length message string, preceded by it's length
                let root_dir = root_dir.as_bytes();
                buf.write_u16::<NetworkEndian>(root_dir.len() as u16)?;
                buf.write_all(&root_dir)?;
                bytes_written += 2 + root_dir.len()
            }
        }
        Ok(bytes_written)
    }
}

impl DeserializePacket for LocalRequest {
    type Output = LocalRequest;

    fn deserialize(mut buf: &mut impl Read) -> io::Result<LocalRequest> {
        // We'll match the same `u8` that is used to recognize which request type this is
        match buf.read_u8()? {
            // Echo
            1 => Ok(LocalRequest::SyncProject { root_dir: extract_string(&mut buf)? }),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid Request Type",
            )),
        }
    }
}
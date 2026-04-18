use anyhow::{anyhow, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum SignatureAlgorithmID {
    #[default]
    VerityRsaPkcs1V15WithSha256 = 0x0421,
}

impl SignatureAlgorithmID {
    pub fn to_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(value: u32) -> Result<Self> {
        match value {
            0x0421 => Ok(Self::VerityRsaPkcs1V15WithSha256),
            _ => Err(anyhow!("Unsupported signature algorithm: {}", value)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum HashAlgorithm {
    #[default]
    SHA256 = 1,
}

impl HashAlgorithm {
    pub fn to_u32(self) -> u32 {
        self as u32
    }

    pub(crate) fn from_read<R: Read>(read: &mut R) -> Result<Self> {
        match read.read_u32::<LittleEndian>()? {
            1 => Ok(Self::SHA256),
            value => Err(anyhow!("Unsupported hash algorithm: {}", value)),
        }
    }
}

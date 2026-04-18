//! API for APK Signature Scheme [v4].
//!
//! [v4]: https://source.android.com/security/apksigning/v4

use anyhow::{anyhow, bail, Context, Result};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{copy, Cursor, Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::algorithms::{HashAlgorithm, SignatureAlgorithmID};
use crate::hashtree::HashTree;

#[derive(Default)]
pub struct V4Signature<R: Read + Seek> {
    pub version: Version,
    pub hashing_info: HashingInfo,
    pub signing_info: SigningInfo,
    pub merkle_tree_size: u32,
    pub merkle_tree_offset: u64,
    data: R,
}

#[derive(Default)]
pub struct HashingInfo {
    pub hash_algorithm: HashAlgorithm,
    pub log2_blocksize: u8,
    pub salt: Box<[u8]>,
    pub raw_root_hash: Box<[u8]>,
}

#[derive(Default)]
pub struct SigningInfo {
    pub apk_digest: Box<[u8]>,
    pub x509_certificate: Box<[u8]>,
    pub additional_data: Box<[u8]>,
    pub public_key: Box<[u8]>,
    pub signature_algorithm_id: SignatureAlgorithmID,
    pub signature: Box<[u8]>,
}

#[derive(Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum Version {
    #[default]
    V2 = 2,
}

impl Version {
    fn from(val: u32) -> Result<Self> {
        match val {
            2 => Ok(Self::V2),
            _ => Err(anyhow!("{} is an unsupported version", val)),
        }
    }

    fn to_u32(&self) -> u32 {
        match self {
            Self::V2 => 2,
        }
    }
}

impl V4Signature<fs::File> {
    pub fn from_idsig_path<P: AsRef<Path>>(idsig_path: P) -> Result<Self> {
        let idsig = fs::File::open(idsig_path).context("Cannot find idsig file")?;
        Self::from_idsig(idsig)
    }
}

impl<R: Read + Seek> V4Signature<R> {
    pub fn from_idsig(mut r: R) -> Result<Self> {
        Ok(Self {
            version: Version::from(r.read_u32::<LittleEndian>()?)?,
            hashing_info: HashingInfo::from(&mut r)?,
            signing_info: SigningInfo::from(&mut r)?,
            merkle_tree_size: r.read_u32::<LittleEndian>()?,
            merkle_tree_offset: r.stream_position()?,
            data: r,
        })
    }

    pub fn create(
        apk: &mut R,
        block_size: usize,
        salt: &[u8],
        algorithm: HashAlgorithm,
    ) -> Result<V4Signature<Cursor<Vec<u8>>>> {
        let start = apk.stream_position()?;
        let size = apk.seek(SeekFrom::End(0))? as usize;
        apk.seek(SeekFrom::Start(start))?;

        let hash_tree = HashTree::from(apk, size, salt, block_size, algorithm)?;
        let apk_digest = compute_apk_digest(apk, start)?;

        Ok(V4Signature {
            version: Version::V2,
            hashing_info: HashingInfo {
                hash_algorithm: algorithm,
                log2_blocksize: log2(block_size),
                salt: salt.into(),
                raw_root_hash: hash_tree.root_hash.into_boxed_slice(),
            },
            signing_info: SigningInfo {
                apk_digest: apk_digest.into_boxed_slice(),
                signature_algorithm_id: SignatureAlgorithmID::VerityRsaPkcs1V15WithSha256,
                ..Default::default()
            },
            merkle_tree_size: hash_tree.tree.len() as u32,
            merkle_tree_offset: 0,
            data: Cursor::new(hash_tree.tree),
        })
    }

    pub fn write_into<W: Write + Seek>(&mut self, mut w: &mut W) -> Result<()> {
        w.write_u32::<LittleEndian>(self.version.to_u32())?;
        self.hashing_info.write_into(&mut w)?;
        self.signing_info.write_into(&mut w)?;
        w.write_u32::<LittleEndian>(self.merkle_tree_size)?;

        self.data.seek(SeekFrom::Start(self.merkle_tree_offset))?;
        let copied_size = copy(&mut self.data, &mut w)?;
        if copied_size != self.merkle_tree_size as u64 {
            bail!(
                "merkle tree is {} bytes, but only {} bytes are written.",
                self.merkle_tree_size,
                copied_size
            );
        }
        Ok(())
    }
}

impl HashingInfo {
    fn from(mut r: &mut dyn Read) -> Result<Self> {
        r.read_u32::<LittleEndian>()?;
        Ok(Self {
            hash_algorithm: HashAlgorithm::from_read(&mut r)?,
            log2_blocksize: r.read_u8()?,
            salt: read_sized_array(&mut r)?,
            raw_root_hash: read_sized_array(&mut r)?,
        })
    }

    fn write_into<W: Write + Seek>(&self, mut w: &mut W) -> Result<()> {
        let start = w.stream_position()?;
        w.write_u32::<LittleEndian>(0)?;
        w.write_u32::<LittleEndian>(self.hash_algorithm.to_u32())?;
        w.write_u8(self.log2_blocksize)?;
        write_sized_array(&mut w, &self.salt)?;
        write_sized_array(&mut w, &self.raw_root_hash)?;
        let end = w.stream_position()?;
        let size = end - start - std::mem::size_of::<u32>() as u64;
        w.seek(SeekFrom::Start(start))?;
        w.write_u32::<LittleEndian>(size as u32)?;
        w.seek(SeekFrom::Start(end))?;
        Ok(())
    }
}

impl SigningInfo {
    fn from(mut r: &mut dyn Read) -> Result<Self> {
        r.read_u32::<LittleEndian>()?;
        Ok(Self {
            apk_digest: read_sized_array(&mut r)?,
            x509_certificate: read_sized_array(&mut r)?,
            additional_data: read_sized_array(&mut r)?,
            public_key: read_sized_array(&mut r)?,
            signature_algorithm_id: SignatureAlgorithmID::from_u32(
                r.read_u32::<LittleEndian>()?,
            )?,
            signature: read_sized_array(&mut r)?,
        })
    }

    fn write_into<W: Write + Seek>(&self, mut w: &mut W) -> Result<()> {
        let start = w.stream_position()?;
        w.write_u32::<LittleEndian>(0)?;
        write_sized_array(&mut w, &self.apk_digest)?;
        write_sized_array(&mut w, &self.x509_certificate)?;
        write_sized_array(&mut w, &self.additional_data)?;
        write_sized_array(&mut w, &self.public_key)?;
        w.write_u32::<LittleEndian>(self.signature_algorithm_id.to_u32())?;
        write_sized_array(&mut w, &self.signature)?;
        let end = w.stream_position()?;
        let size = end - start - std::mem::size_of::<u32>() as u64;
        w.seek(SeekFrom::Start(start))?;
        w.write_u32::<LittleEndian>(size as u32)?;
        w.seek(SeekFrom::Start(end))?;
        Ok(())
    }
}

fn read_sized_array(r: &mut dyn Read) -> Result<Box<[u8]>> {
    let size = r.read_u32::<LittleEndian>()?;
    let mut data = vec![0; size as usize];
    r.read_exact(&mut data)?;
    Ok(data.into_boxed_slice())
}

fn write_sized_array(w: &mut dyn Write, data: &[u8]) -> Result<()> {
    w.write_u32::<LittleEndian>(data.len() as u32)?;
    Ok(w.write_all(data)?)
}

fn log2(n: usize) -> u8 {
    let num_bits = std::mem::size_of::<usize>() * 8;
    (num_bits as u32 - n.leading_zeros() - 1) as u8
}

fn compute_apk_digest<R: Read + Seek>(apk: &mut R, start: u64) -> Result<Vec<u8>> {
    apk.seek(SeekFrom::Start(start))?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    loop {
        let read = apk.read(&mut buf)?;
        if read == 0 {
            break;
        }
        hasher.update(&buf[..read]);
    }
    apk.seek(SeekFrom::Start(start))?;
    Ok(hasher.finalize().to_vec())
}

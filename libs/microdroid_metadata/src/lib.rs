use std::collections::BTreeMap;
use std::io::{Read, Result, Write};

mod generated {
    include!(concat!(env!("OUT_DIR"), "/metadata_sanitized.rs"));
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ApexPayload {
    pub name: String,
    pub partition_name: String,
    pub last_update_seconds: u64,
    pub is_factory: bool,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ApkPayload {
    pub name: String,
    pub payload_partition_name: String,
    pub idsig_partition_name: String,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PayloadConfig {
    pub payload_binary_name: String,
    pub extra_apk_count: u32,
    pub special_fields: BTreeMap<String, String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PayloadMetadata {
    Config(PayloadConfig),
    ConfigPath(String),
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    pub version: u32,
    pub apexes: Vec<ApexPayload>,
    pub apk: Option<ApkPayload>,
    pub payload: Option<PayloadMetadata>,
}

impl From<&ApexPayload> for generated::ApexPayload {
    fn from(value: &ApexPayload) -> Self {
        Self {
            name: value.name.clone(),
            partition_name: value.partition_name.clone(),
            last_update_seconds: value.last_update_seconds,
            is_factory: value.is_factory,
            ..Default::default()
        }
    }
}

impl From<&ApkPayload> for generated::ApkPayload {
    fn from(value: &ApkPayload) -> Self {
        Self {
            name: value.name.clone(),
            payload_partition_name: value.payload_partition_name.clone(),
            idsig_partition_name: value.idsig_partition_name.clone(),
            ..Default::default()
        }
    }
}

impl From<&PayloadConfig> for generated::PayloadConfig {
    fn from(value: &PayloadConfig) -> Self {
        Self {
            payload_binary_name: value.payload_binary_name.clone(),
            extra_apk_count: value.extra_apk_count,
            ..Default::default()
        }
    }
}

impl From<&Metadata> for generated::Metadata {
    fn from(value: &Metadata) -> Self {
        Self {
            version: value.version,
            apexes: value.apexes.iter().map(Into::into).collect(),
            apk: value.apk.as_ref().map(Into::into).into(),
            payload: value.payload.as_ref().map(|payload| match payload {
                PayloadMetadata::Config(config) => generated::metadata::Payload::Config(config.into()),
                PayloadMetadata::ConfigPath(path) => {
                    generated::metadata::Payload::ConfigPath(path.clone())
                }
            }),
            ..Default::default()
        }
    }
}

impl From<generated::ApexPayload> for ApexPayload {
    fn from(value: generated::ApexPayload) -> Self {
        Self {
            name: value.name,
            partition_name: value.partition_name,
            last_update_seconds: value.last_update_seconds,
            is_factory: value.is_factory,
        }
    }
}

impl From<generated::ApkPayload> for ApkPayload {
    fn from(value: generated::ApkPayload) -> Self {
        Self {
            name: value.name,
            payload_partition_name: value.payload_partition_name,
            idsig_partition_name: value.idsig_partition_name,
        }
    }
}

impl From<generated::PayloadConfig> for PayloadConfig {
    fn from(value: generated::PayloadConfig) -> Self {
        Self {
            payload_binary_name: value.payload_binary_name,
            extra_apk_count: value.extra_apk_count,
            special_fields: BTreeMap::new(),
        }
    }
}

impl From<generated::Metadata> for Metadata {
    fn from(value: generated::Metadata) -> Self {
        Self {
            version: value.version,
            apexes: value.apexes.into_iter().map(Into::into).collect(),
            apk: value.apk.into_option().map(Into::into),
            payload: value.payload.map(|payload| match payload {
                generated::metadata::Payload::Config(config) => PayloadMetadata::Config(config.into()),
                generated::metadata::Payload::ConfigPath(path) => PayloadMetadata::ConfigPath(path),
            }),
        }
    }
}

pub fn write_metadata(metadata: &Metadata, writer: &mut impl Write) -> Result<()> {
    let metadata: generated::Metadata = metadata.into();
    let mut body = Vec::new();
    protobuf::Message::write_to_vec(&metadata, &mut body).map_err(std::io::Error::other)?;
    writer.write_all(&(body.len() as i32).to_be_bytes())?;
    writer.write_all(&body)
}

pub fn read_metadata(reader: impl Read) -> Result<Metadata> {
    let mut reader = reader;
    let metadata = <generated::Metadata as protobuf::Message>::parse_from_reader(&mut reader)
        .map_err(std::io::Error::other)?;
    Ok(metadata.into())
}

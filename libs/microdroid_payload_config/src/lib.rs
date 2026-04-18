use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ApexConfig {
    pub name: String,
}

/// `os` block in payload JSON (legacy field; name may be empty or "microdroid").
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OsDesc {
    #[serde(default)]
    pub name: String,
}

impl Default for OsDesc {
    fn default() -> Self {
        Self { name: "microdroid".to_string() }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VmPayloadConfig {
    #[serde(default)]
    pub os: OsDesc,
    pub apexes: Vec<ApexConfig>,
    #[serde(default)]
    pub prefer_staged: bool,
    #[serde(default)]
    pub extra_apks: Vec<ApkConfig>,
    #[serde(default)]
    pub hugepages: bool,
    #[serde(default)]
    pub task: Option<Task>,
}

impl Default for VmPayloadConfig {
    fn default() -> Self {
        Self {
            os: OsDesc::default(),
            apexes: vec![],
            prefer_staged: false,
            extra_apks: vec![],
            hugepages: false,
            task: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApkConfig {
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Task {
    #[serde(rename = "type")]
    pub type_: TaskType,
    pub command: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TaskType {
    MicrodroidLauncher,
}

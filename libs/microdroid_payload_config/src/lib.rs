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
    #[serde(default)]
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
    #[serde(default, rename = "type")]
    pub type_: TaskType,
    pub command: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum TaskType {
    #[serde(rename = "executable")]
    #[default]
    Executable,
    #[serde(rename = "microdroid_launcher")]
    MicrodroidLauncher,
}

#[cfg(test)]
mod tests {
    use super::{TaskType, VmPayloadConfig};

    #[test]
    fn parses_minimal_microdroid_launcher_config() {
        let config: VmPayloadConfig = serde_json::from_str(
            r#"{
                "task": {
                    "type": "microdroid_launcher",
                    "command": "MicrodroidEmptyPayloadJniLib.so"
                }
            }"#,
        )
        .expect("minimal launcher config should parse");

        assert!(config.apexes.is_empty());
        assert!(config.extra_apks.is_empty());
        assert!(matches!(config.task.expect("task").type_, TaskType::MicrodroidLauncher));
    }

    #[test]
    fn parses_executable_and_defaults_missing_task_type() {
        let executable: VmPayloadConfig = serde_json::from_str(
            r#"{
                "task": {
                    "type": "executable",
                    "command": "/bin/payload"
                }
            }"#,
        )
        .expect("executable config should parse");
        assert!(matches!(executable.task.expect("task").type_, TaskType::Executable));

        let defaulted: VmPayloadConfig = serde_json::from_str(
            r#"{
                "task": {
                    "command": "/bin/default-payload"
                }
            }"#,
        )
        .expect("missing task type should default to executable");
        assert!(matches!(defaulted.task.expect("task").type_, TaskType::Executable));
    }
}

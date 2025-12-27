use serde::{Deserialize, Serialize};

/// Represents an argument or return value
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteArgument {
    #[serde(rename = "type")]
    pub arg_type: String,
    pub value: String,
}

/// Event payload for remote spy call
#[derive(Debug, Clone, Serialize)]
pub struct RemoteCallEvent {
    #[serde(rename = "callId")]
    pub call_id: u32,
    #[serde(rename = "remoteId")]
    pub remote_id: u32,
    pub name: String,
    pub path: String,
    pub class: String,
    pub direction: String,
    pub timestamp: String,
    pub arguments: Vec<RemoteArgument>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "returnValues")]
    pub return_values: Option<Vec<RemoteArgument>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "callingScriptName")]
    pub calling_script_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "callingScriptPath")]
    pub calling_script_path: Option<String>,
}

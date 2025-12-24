use serde::{Deserialize, Serialize};

/// Represents a single remote call event from the client
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteCallEvent {
    /// Unique numeric ID for the remote instance
    #[serde(rename = "remoteId")]
    pub remote_id: u32,

    /// Remote name (e.g., "UpdateInventory")
    pub name: String,

    /// Full path (e.g., "ReplicatedStorage.Remotes.Inventory.Update")
    pub path: String,

    /// Remote type: "RemoteEvent" or "RemoteFunction"
    #[serde(rename = "remoteType")]
    pub remote_type: String,

    /// Call direction: "outgoing" or "incoming"
    pub direction: String,

    /// Timestamp (ISO 8601 string)
    pub timestamp: String,

    /// Call arguments
    pub arguments: Vec<RemoteArgument>,

    /// Return value (only for RemoteFunctions with incoming direction)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "returnValue")]
    pub return_value: Option<RemoteArgument>,

    /// Calling script name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "callingScript")]
    pub calling_script: Option<String>,

    /// Calling script path
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "callingScriptPath")]
    pub calling_script_path: Option<String>,
}

/// Represents an argument or return value
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteArgument {
    #[serde(rename = "type")]
    pub arg_type: String,
    pub value: String,
}

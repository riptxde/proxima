use crate::models::explorer::*;
use crate::models::remote_spy::*;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// Custom deserializer for HashMap that handles empty arrays from Lua
pub(super) fn deserialize_props<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, PropertyData>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Object(map) => {
            serde_json::from_value(Value::Object(map)).map_err(serde::de::Error::custom)
        }
        Value::Array(arr) if arr.is_empty() => Ok(HashMap::new()),
        _ => Err(serde::de::Error::custom(
            "expected object or empty array for props",
        )),
    }
}

/// Messages sent from WebSocket clients to server
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub(super) enum ClientMessage {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "register")]
    Register { username: String },
    #[serde(rename = "pong")]
    Pong,
    #[serde(rename = "log")]
    Log { level: u8, message: String },
    #[serde(rename = "exp_tree")]
    ExpTree { nodes: Vec<ExplorerNode> },
    #[serde(rename = "exp_properties")]
    ExpProperties {
        id: u32,
        #[serde(deserialize_with = "deserialize_props")]
        props: HashMap<String, PropertyData>,
        #[serde(rename = "specialProps", deserialize_with = "deserialize_props")]
        special_props: HashMap<String, PropertyData>,
    },
    #[serde(rename = "exp_search_results")]
    ExpSearchResults {
        query: String,
        results: Vec<SearchResult>,
        total: u32,
        limited: bool,
    },
    #[serde(rename = "exp_tree_changed")]
    ExpTreeChanged,
    #[serde(rename = "exp_decompiled")]
    ExpDecompiled { id: u32, source: String },
    #[serde(rename = "rspy_call")]
    RspyCall {
        #[serde(rename = "callId")]
        call_id: u32,
        #[serde(rename = "remoteId")]
        remote_id: u32,
        name: String,
        path: String,
        class: String,
        direction: String,
        timestamp: String,
        arguments: Vec<RemoteArgument>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "returnValues")]
        return_values: Option<Vec<RemoteArgument>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "callingScriptName")]
        calling_script_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "callingScriptPath")]
        calling_script_path: Option<String>,
    },
    #[serde(rename = "rspy_decompiled")]
    RspyDecompiled {
        #[serde(rename = "callId")]
        call_id: u32,
        source: String,
    },
    #[serde(rename = "rspy_generated_code")]
    RspyGeneratedCode {
        #[serde(rename = "callId")]
        call_id: u32,
        code: String,
    },
}

/// Messages sent from server to WebSocket clients
#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub(super) enum ServerMessage {
    #[serde(rename = "exec")]
    Exec { script: String, redirect: bool },
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "exp_start")]
    ExpStart,
    #[serde(rename = "exp_stop")]
    ExpStop,
    #[serde(rename = "exp_decompile")]
    ExpDecompile { id: u32 },
    #[serde(rename = "rspy_start")]
    RspyStart,
    #[serde(rename = "rspy_stop")]
    RspyStop,
    #[serde(rename = "rspy_decompile")]
    RspyDecompile {
        #[serde(rename = "callId")]
        call_id: u32,
    },
    #[serde(rename = "rspy_generate_code")]
    RspyGenerateCode {
        #[serde(rename = "callId")]
        call_id: u32,
    },
}

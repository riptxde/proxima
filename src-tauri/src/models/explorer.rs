use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a node in the explorer tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorerNode {
    pub id: u32,
    pub n: String, // name
    pub c: String, // className
    pub h: bool,   // hasChildren
    pub children: Vec<ExplorerNode>,
}

/// Property data for an instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyData {
    pub value: String,
    pub r#type: String,
    pub class: String,
    pub deprecated: bool,
    pub hidden: bool,
    #[serde(rename = "notScriptable")]
    pub not_scriptable: bool,
}

/// Search result item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: u32,
    pub n: String,
    pub c: String,
    pub path: Vec<u32>,
    pub h: bool,
}

/// Event payloads for frontend communication
#[derive(Debug, Clone, Serialize)]
pub struct TreeEvent {
    pub nodes: Vec<ExplorerNode>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PropertiesEvent {
    pub id: u32,
    pub props: HashMap<String, PropertyData>,
    #[serde(rename = "specialProps")]
    pub special_props: HashMap<String, PropertyData>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchResultsEvent {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub total: u32,
    pub limited: bool,
}

/// Messages sent to client
#[derive(Debug, Serialize, Deserialize)]
pub struct GetExplorerTreeMessage {
    pub r#type: String,
    #[serde(rename = "expandedIds")]
    pub expanded_ids: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetExplorerPropertiesMessage {
    pub r#type: String,
    pub id: u32,
    pub properties: Vec<String>,
    #[serde(rename = "specialProperties")]
    pub special_properties: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchExplorerMessage {
    pub r#type: String,
    pub query: String,
    #[serde(rename = "searchIn")]
    pub search_in: String,
    #[serde(rename = "maxResults")]
    pub max_results: u32,
}

impl GetExplorerTreeMessage {
    pub fn new(expanded_ids: Vec<u32>) -> Self {
        Self {
            r#type: "get_explorer_tree".to_string(),
            expanded_ids,
        }
    }
}

impl GetExplorerPropertiesMessage {
    pub fn new(id: u32, properties: Vec<String>, special_properties: Vec<String>) -> Self {
        Self {
            r#type: "get_explorer_properties".to_string(),
            id,
            properties,
            special_properties,
        }
    }
}

impl SearchExplorerMessage {
    pub fn new(query: String, search_in: String, max_results: u32) -> Self {
        Self {
            r#type: "search_explorer".to_string(),
            query,
            search_in,
            max_results,
        }
    }
}

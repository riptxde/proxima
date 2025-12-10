use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDump {
    #[serde(rename = "Classes")]
    pub classes: Vec<ApiClass>,
    #[serde(rename = "Version")]
    pub version: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiClass {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Members")]
    pub members: Vec<ApiMember>,
    #[serde(rename = "Superclass")]
    pub superclass: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "MemberType")]
pub enum ApiMember {
    Property {
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "ValueType")]
        value_type: ApiValueType,
        #[serde(rename = "Tags")]
        tags: Option<Vec<String>>,
    },
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiValueType {
    #[serde(rename = "Name")]
    pub name: String,
}

pub struct ApiDumpService {
    dump: Option<ApiDump>,
    class_lookup: HashMap<String, ApiClass>,
}

impl ApiDumpService {
    pub fn new() -> Self {
        Self {
            dump: None,
            class_lookup: HashMap::new(),
        }
    }

    pub async fn load(&mut self) -> Result<(), String> {
        log::info!("Loading Roblox API dump...");

        let version = self.fetch_version().await?;
        log::info!("Roblox version: {}", version);

        let dump = self.fetch_api_dump(&version).await?;
        log::info!(
            "API dump loaded successfully ({} classes)",
            dump.classes.len()
        );

        self.class_lookup = self.build_class_lookup(&dump);
        self.dump = Some(dump);

        Ok(())
    }

    async fn fetch_version(&self) -> Result<String, String> {
        let version_url = "https://raw.githubusercontent.com/MaximumADHD/Roblox-Client-Tracker/refs/heads/roblox/version-guid.txt";

        reqwest::get(version_url)
            .await
            .map_err(|e| format!("Failed to fetch version: {}", e))?
            .text()
            .await
            .map(|s| s.trim().to_string())
            .map_err(|e| format!("Failed to read version: {}", e))
    }

    async fn fetch_api_dump(&self, version: &str) -> Result<ApiDump, String> {
        let api_url = format!("https://setup.rbxcdn.com/{}-API-Dump.json", version);
        log::info!("Fetching API dump from: {}", api_url);

        let api_text = reqwest::get(&api_url)
            .await
            .map_err(|e| format!("Failed to fetch API dump: {}", e))?
            .text()
            .await
            .map_err(|e| format!("Failed to read API dump: {}", e))?;

        log::info!("API dump fetched ({} bytes), parsing...", api_text.len());

        serde_json::from_str(&api_text).map_err(|e| format!("Failed to parse API dump: {}", e))
    }

    fn build_class_lookup(&self, dump: &ApiDump) -> HashMap<String, ApiClass> {
        dump.classes
            .iter()
            .map(|class| (class.name.clone(), class.clone()))
            .collect()
    }

    pub fn get_class_properties(&self, class_name: &str) -> (Vec<String>, Vec<String>) {
        let mut properties = Vec::new();
        let mut special_properties = Vec::new();
        let mut current_class_name = Some(class_name.to_string());

        while let Some(name) = current_class_name {
            if let Some(class) = self.class_lookup.get(&name) {
                self.collect_class_properties(class, &mut properties, &mut special_properties);
                current_class_name = class.superclass.clone().filter(|s| s != "<<<ROOT>>>");
            } else {
                break;
            }
        }

        (properties, special_properties)
    }

    fn collect_class_properties(
        &self,
        class: &ApiClass,
        properties: &mut Vec<String>,
        special_properties: &mut Vec<String>,
    ) {
        for member in &class.members {
            if let ApiMember::Property { name, tags, .. } = member {
                if self.should_skip_property(name) {
                    continue;
                }

                if self.is_special_property(tags) {
                    special_properties.push(name.clone());
                } else {
                    properties.push(name.clone());
                }
            }
        }
    }

    fn should_skip_property(&self, name: &str) -> bool {
        matches!(name, "UniqueId" | "Capabilities")
    }

    fn is_special_property(&self, tags: &Option<Vec<String>>) -> bool {
        tags.as_ref().map_or(false, |tags| {
            tags.contains(&"Hidden".to_string()) || tags.contains(&"NotScriptable".to_string())
        })
    }

    pub fn is_loaded(&self) -> bool {
        self.dump.is_some()
    }
}

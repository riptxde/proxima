use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyMetadata {
    pub name: String,
    #[serde(rename = "valueType")]
    pub value_type: Option<String>,
    pub deprecated: bool,
    pub hidden: bool,
    pub not_scriptable: bool,
}

/// MessagePack API dump structure from roblox_api_viewer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDumpMsgpack {
    pub metadata: ApiMetadata,
    pub classes: Vec<ApiClassMsgpack>,
    pub enums: Vec<ApiEnum>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMetadata {
    pub version: Option<String>,
    pub updated: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiClassMsgpack {
    pub name: String,
    pub inherits: Option<Vec<String>>,
    pub members: Vec<ApiMemberMsgpack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMemberMsgpack {
    pub name: String,
    pub member_type: String,
    #[serde(default)]
    pub value_type: Option<String>,
    #[serde(default)]
    pub security: Option<String>,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub unreplicated: bool,
    #[serde(default)]
    pub unscriptable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEnum {
    pub name: String,
    pub items: Vec<ApiEnumItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEnumItem {
    pub name: String,
    pub value: i32,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub unscriptable: bool,
    #[serde(default)]
    pub security: Option<String>,
}

pub struct ApiDumpService {
    dump: Option<ApiDumpMsgpack>,
    class_lookup: HashMap<String, ApiClassMsgpack>,
}

impl ApiDumpService {
    pub fn new() -> Self {
        Self {
            dump: None,
            class_lookup: HashMap::new(),
        }
    }

    pub async fn load(&mut self) -> Result<(), String> {
        log::info!("Loading Roblox API dump from roblox_api_viewer...");

        let dump = self.fetch_api_dump().await?;

        log::info!(
            "API dump loaded successfully ({} classes, {} enums)",
            dump.classes.len(),
            dump.enums.len()
        );

        if let Some(ref metadata) = dump.metadata.version {
            log::info!("API version: {}", metadata);
        }
        if let Some(ref updated) = dump.metadata.updated {
            log::info!("API updated: {}", updated);
        }

        self.class_lookup = self.build_class_lookup(&dump);
        self.dump = Some(dump);

        Ok(())
    }

    async fn fetch_api_dump(&self) -> Result<ApiDumpMsgpack, String> {
        let api_url = "https://github.com/riptxde/roblox_api_viewer/raw/refs/heads/master/resources/roblox_api.msgpack";

        let bytes = reqwest::get(api_url)
            .await
            .map_err(|e| format!("Failed to fetch API dump: {}", e))?
            .bytes()
            .await
            .map_err(|e| format!("Failed to read API dump bytes: {}", e))?;

        rmp_serde::from_slice(&bytes)
            .map_err(|e| format!("Failed to parse MessagePack API dump: {}", e))
    }

    fn build_class_lookup(&self, dump: &ApiDumpMsgpack) -> HashMap<String, ApiClassMsgpack> {
        dump.classes
            .iter()
            .map(|class| (class.name.clone(), class.clone()))
            .collect()
    }

    pub fn get_class_properties(
        &self,
        class_name: &str,
    ) -> (Vec<PropertyMetadata>, Vec<PropertyMetadata>) {
        let mut properties = Vec::new();
        let mut special_properties = Vec::new();

        // Walk the inheritance chain to collect all properties
        self.collect_properties_with_inheritance(
            class_name,
            &mut properties,
            &mut special_properties,
        );

        (properties, special_properties)
    }

    fn collect_properties_with_inheritance(
        &self,
        class_name: &str,
        properties: &mut Vec<PropertyMetadata>,
        special_properties: &mut Vec<PropertyMetadata>,
    ) {
        // Get the class from the lookup
        let Some(class) = self.class_lookup.get(class_name) else {
            return;
        };

        // First, recursively collect properties from parent classes
        if let Some(ref inherits_list) = class.inherits {
            for parent_class in inherits_list {
                self.collect_properties_with_inheritance(
                    parent_class,
                    properties,
                    special_properties,
                );
            }
        }

        // Then collect this class's own properties
        self.collect_class_properties(class, properties, special_properties);
    }

    fn collect_class_properties(
        &self,
        class: &ApiClassMsgpack,
        properties: &mut Vec<PropertyMetadata>,
        special_properties: &mut Vec<PropertyMetadata>,
    ) {
        for member in &class.members {
            // Only process Property members
            if member.member_type != "Property" {
                continue;
            }

            if self.should_skip_property(&member.name) {
                continue;
            }

            let metadata = PropertyMetadata {
                name: member.name.clone(),
                value_type: member.value_type.clone(),
                deprecated: member.deprecated,
                hidden: member.hidden,
                not_scriptable: member.unscriptable,
            };

            // Properties are "special" if they have:
            // - hidden tag
            // - unscriptable tag
            // - security restrictions (anything other than "None")
            let has_security = member
                .security
                .as_ref()
                .map(|s| s != "None")
                .unwrap_or(false);

            if member.hidden || member.unscriptable || has_security {
                special_properties.push(metadata);
            } else {
                properties.push(metadata);
            }
        }
    }

    fn should_skip_property(&self, name: &str) -> bool {
        matches!(name, "UniqueId" | "Capabilities")
    }

    pub fn is_loaded(&self) -> bool {
        self.dump.is_some()
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ResourceType {
    Organization,
    Endpoint,
}

impl ResourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceType::Organization => "Organization",
            ResourceType::Endpoint => "Endpoint",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Organization" => Some(ResourceType::Organization),
            "Endpoint" => Some(ResourceType::Endpoint),
            _ => None,
        }
    }
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct Resource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>, // to be resoloved later

    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_rules: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct DomainResource {
    #[serde(flatten)]
    pub resource: Resource,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contained: Option<Vec<String>>, // To be resolved later

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>, // to be resolved later
}

pub trait ResourceType {
    const TYPE: &'static str;

    fn get_resource_type() -> String {
        Self::TYPE.to_string()
    }
}

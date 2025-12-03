use serde::{Deserialize, Serialize};

use crate::r4::resources::ResourceType;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Element {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extention: Option<Vec<String>>, // to be resolved later
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct BackboneElement {
    #[serde(flatten)]
    pub element: Element,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_extensions: Option<Vec<String>>, // to be resolved later
}

impl ResourceType for Element {
    const TYPE: &'static str = "Element";
}

impl ResourceType for BackboneElement {
    const TYPE: &'static str = "BackboneElement";
}

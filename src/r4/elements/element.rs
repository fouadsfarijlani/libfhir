use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Element {
    pub id: Option<String>,
    pub extention: Option<Vec<String>>, // to be resolved later
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackboneElement {
    #[serde(flatten)]
    pub element: Element,
    pub modifier_extensions: Option<Vec<String>>, // to be resolved later
}

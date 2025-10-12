use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Element {
    pub id: Option<String>,
    pub extention: Option<Vec<String>>, // to be resolved later
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackboneElement {
    #[serde(flatten)]
    pub element: Element,
    pub modifier_extensions: Option<Vec<String>>, // to be resolved later
}

use serde::{Deserialize, Serialize};

use crate::elements::{complex_types::Identifier, element::Element};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Reference {
    #[serde(flatten)]
    pub element: Option<Element>,
    pub reference: Option<String>,
    pub r#type: Option<String>,
    pub display: Option<String>,
    pub identifier: Option<Vec<Identifier>>,
}

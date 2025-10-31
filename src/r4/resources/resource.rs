use serde::{Deserialize, Serialize};

use crate::elements::reference::Reference;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Resource {
    pub id: Option<String>,
    pub meta: Option<String>, // to be resoloved later
    pub implicit_rules: Option<String>,
}

impl Resource {
    pub fn new(id: String, meta: String, implicit_rule: String) -> Self {
        Self {
            id: Some(id),
            meta: Some(meta),
            implicit_rules: Some(implicit_rule),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub text: Option<String>,
    pub contained: Option<Vec<String>>, // To be resolved later
    pub exnetions: Option<Vec<String>>, // to be resolved later
}

pub trait GetReferences {
    fn get_references(&self) -> Vec<&Reference>;
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Resource {
    pub id: Option<String>,
    pub meta: Option<String>, // to be resoloved later
    pub implicit_rules: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct DomainResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub text: Option<String>,
    pub contained: Option<Vec<String>>, // To be resolved later
    pub exnetions: Option<Vec<String>>, // to be resolved later
}

pub trait ResourceType {
    const TYPE: &'static str;

    fn get_resource_type(&self) -> &'static str {
        Self::TYPE
    }
}

use std::panic;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct Resource {
    pub id: Option<String>,
    pub meta: Option<String>, // to be resoloved later
    pub implicit_rules: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
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

// TODO: these functions should be in a separate crate
pub fn from_json<'a, T>(data: &'a str) -> T
where
    T: ResourceType,
    T: Deserialize<'a>,
{
    let results = serde_json::from_str(data);
    match results {
        Ok(res) => res,
        Err(e) => panic!("{e:?}"),
    }
}

use serde::{Deserialize, Serialize};
use std::convert::From;

use crate::{
    elements::{
        complex_types::{Address, CodeableConcept, ContactPoint, HumanName, Identifier},
        element::BackboneElement,
        reference::Reference,
    },
    resources::resource::{DomainResource, GetReferences},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationContact {
    #[serde(flatten)]
    pub element: BackboneElement,
    pub purpose: Option<CodeableConcept>,
    pub name: Option<HumanName>,
    pub telecom: Option<Vec<ContactPoint>>,
    pub address: Option<Address>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[serde(flatten)]
    pub resource: Option<DomainResource>,
    pub identifier: Option<Vec<Identifier>>,
    pub active: Option<bool>,
    pub r#type: Option<Vec<CodeableConcept>>,
    pub name: Option<String>,
    pub alias: Option<Vec<String>>,
    pub telecom: Option<Vec<ContactPoint>>,
    pub address: Option<Vec<Address>>,
    pub part_of: Option<Reference>,
    pub contact: Option<Vec<OrganizationContact>>,
    pub endpoint: Option<Vec<Reference>>,
}

impl Organization {
    pub fn from_json(data: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(data)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl From<&str> for Organization {
    fn from(value: &str) -> Self {
        let result = Self::from_json(value);
        match result {
            Ok(org) => org,
            Err(e) => panic!("{e:?}"),
        }
    }
}

impl GetReferences for Organization {
    fn get_references(&self) -> Vec<&Reference> {
        let mut references: Vec<&Reference> = Vec::new();
        if let Some(eps) = &self.endpoint {
            references = eps.iter().collect();
        }

        if let Some(part_of) = &self.part_of {
            references.push(part_of);
        }

        references
    }
}

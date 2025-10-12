use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        complex_types::{Address, CodeableConcept, ContactPoint, HumanName, Identifier},
        element::BackboneElement,
        reference::Reference,
    },
    resources::resource::DomainResource,
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
    pub endpoint: Option<Vec<Reference>>, // to be resolved later
}

impl Organization {
    pub fn from_json(data: &str) -> Self {
        let results = serde_json::from_str::<Organization>(data);
        match results {
            Ok(org) => org,
            Err(e) => panic!("{e:?}"),
        }
    }

    // this can possibly be a trait
    pub fn get_references(&self) -> Vec<&Reference> {
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

use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        Address, BackboneElement, CodeableConcept, ContactPoint, GetResourceRefernces, HumanName,
        Identifier, Reference, ReferenceTypes,
    },
    resources::{DomainResource, Endpoint, ResourceType},
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
    pub part_of: Option<Reference<Organization>>,
    pub contact: Option<Vec<OrganizationContact>>,
    pub endpoint: Option<Vec<Reference<Endpoint>>>, // to be resolved later
}

impl Organization {
    pub fn from_json(data: &str) -> Self {
        let results = serde_json::from_str::<Organization>(data);
        match results {
            Ok(org) => org,
            Err(e) => panic!("{e:?}"),
        }
    }
}

impl ResourceType for Organization {
    const TYPE: &'static str = "Organization";
}

impl GetResourceRefernces for Organization {
    fn get_references(&self) -> Vec<ReferenceTypes> {
        let mut references: Vec<ReferenceTypes> = Vec::new();

        if let Some(eps) = &self.endpoint {
            references = eps.into_iter().map(|e| ReferenceTypes::from(e)).collect();
        }

        if let Some(part_of) = &self.part_of {
            references.push(ReferenceTypes::from(part_of));
        }

        references
    }
}

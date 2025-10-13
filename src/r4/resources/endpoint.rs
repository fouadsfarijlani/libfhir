use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        complex_types::{CodeableConcept, Coding, ContactPoint, Identifier, Period},
        reference::Reference,
    },
    resources::resource::{DomainResource, GetReferences},
};

#[derive(Debug, Serialize, Deserialize)]
pub enum EndpointStatus {
    #[serde(rename = "active")]
    Active,

    #[serde(rename = "suspended")]
    Suspended,

    #[serde(rename = "error")]
    Error,

    #[serde(rename = "off")]
    Off,

    #[serde(rename = "entered-in-error")]
    EnteredInError,

    #[serde(rename = "test")]
    Test,
}

impl EndpointStatus {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Self::Active => "active",
            Self::Suspended => "suspended",
            Self::Error => "error",
            Self::Off => "off",
            Self::EnteredInError => "entered-in-error",
            Self::Test => "test",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    #[serde(flatten)]
    pub resource: DomainResource,
    pub identifier: Option<Vec<Identifier>>,
    pub status: EndpointStatus, // to be resolved later
    pub connection_type: Coding,
    pub managing_organization: Option<Reference>,
    pub contact: Option<Vec<ContactPoint>>,
    pub period: Option<Period>,
    pub payload_type: Vec<CodeableConcept>,
    pub payload_mime_type: Option<Vec<String>>, // to be resolved later
    pub address: String,                        // to be resolved later
    pub header: Option<String>,
}

impl Endpoint {
    pub fn from_json(data: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(data)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl From<&str> for Endpoint {
    fn from(value: &str) -> Self {
        let results = serde_json::from_str(value);
        match results {
            Ok(endpoint) => endpoint,
            Err(e) => panic!("{e:?}"),
        }
    }
}

impl GetReferences for Endpoint {
    fn get_references(&self) -> Vec<&Reference> {
        let mut references: Vec<&Reference> = Vec::new();
        if let Some(managing_org) = &self.managing_organization {
            references.push(managing_org);
        }

        references
    }
}

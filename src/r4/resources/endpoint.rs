use crate::{
    elements::{CodeableConcept, Coding, ContactPoint, Identifier, Period, Reference},
    resources::{DomainResource, Organization, ResourceType},
};

#[derive(Debug)]
pub struct Endpoint {
    pub resource: DomainResource,
    pub identifier: Option<Vec<Identifier>>,
    pub status: String, // to be resolved later
    pub connection_type: Coding,
    pub managing_organization: Option<Reference<Organization>>,
    pub contact: Option<Vec<ContactPoint>>,
    pub period: Option<Period>,
    pub payload_type: Vec<CodeableConcept>,
    pub payload_mime_type: Option<Vec<String>>, // to be resolved later
    pub address: String,                        // to be resolved later
    pub header: Option<String>,
}

impl ResourceType for Endpoint {
    const TYPE: &'static str = "Endpoint";
}

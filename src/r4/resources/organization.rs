use crate::CodeableConcept;
use crate::ContactPoint;
use crate::DomainResources;
use crate::Identifier;
use crate::Reference;

#[derive(Debug)]
pub struct Organization {
    resource: Option<DomainResources>,
    identifier: Option<Vec<Identifier>>,
    active: Option<bool>,
    r#type: Option<Vec<CodeableConcept>>,
    name: Option<String>,
    alias: Option<Vec<String>>,
    telecom: Option<Vec<ContactPoint>>, // to be resolved later

    // Complex: Address [0..*]
    address: Option<Vec<String>>, // to be resolved later

    // Complex: Reference (Organization) [0..1]
    part_of: Option<String>, // to be resolved later

    // Complex: OrganizationContact [0..*]
    contact: Option<Vec<String>>, // to be resolved later

    // Complex: Reference (Endpoint) [0..*]
    endpoint: Option<Vec<Reference>>, // to be resolved later
}

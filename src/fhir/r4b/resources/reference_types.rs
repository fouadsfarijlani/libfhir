use serde::{Deserialize, Serialize};
use crate::fhir::r4b::endpoint::Endpoint;
use crate::fhir::r4b::organization::Organization;
use crate::fhir::r4b::resources::Reference;

/// We can have references to different kind of resources (maybe even combined, like org + org_affiliation?)

pub trait ResourceKind {
    const TYPE: &'static str;
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OrganizationKind;

impl ResourceKind for OrganizationKind {
    const TYPE: &'static str = "Organization";
}

impl From<&Organization> for Reference<OrganizationKind> {
    fn from(org: &Organization) -> Self {
        let id = org.id.clone().expect("Endpoint must have an id to build a reference");
        Reference::<OrganizationKind>::to_id(id).with_display(org.name.clone().unwrap_or_default())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EndpointKind;

impl ResourceKind for EndpointKind {
    const TYPE: &'static str = "Endpoint";
}

impl From<&Endpoint> for Reference<EndpointKind> {
    fn from(ep: &Endpoint) -> Self {
        let id = ep.id.clone().expect("Endpoint must have an id to build a reference");
        Reference::<EndpointKind>::to_id(id).with_display(ep.name.clone().unwrap_or_default())
    }
}



use serde::{Deserialize, Serialize};

use crate::{
    elements::{CodeableConcept, ContactPoint, Identifier, Period, Reference},
    resources::{
        self, DomainResource, Endpoint, HealthcareService, Location, Organization, Practitioner,
        ResourceType,
    },
};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct PractitionerRole {
    #[serde(flatten)]
    pub domain_resource: DomainResource,
    pub identifier: Option<Vec<Identifier>>,
    pub active: Option<bool>,
    pub period: Option<Period>,
    pub practitioner: Reference<Practitioner>,
    pub organization: Reference<Organization>,
    pub code: Option<Vec<CodeableConcept>>,
    pub speciality: Option<Vec<CodeableConcept>>,
    pub location: Option<Vec<Reference<Location>>>,
    pub healthcare_service: Option<Vec<Reference<HealthcareService>>>,
    pub telecom: Option<Vec<ContactPoint>>,
    // available time
    // ...
    // ...
    pub endpoint: Option<Vec<Reference<Endpoint>>>,
}

impl ResourceType for PractitionerRole {
    const TYPE: &'static str = "PractitionerRole";
}

impl PractitionerRole {
    pub fn from_json(data: &str) -> Self {
        resources::from_json(data)
    }
}

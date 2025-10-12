use crate::{
    elements::{
        complex_types::{CodeableConcept, ContactPoint, Period},
        reference::Reference,
    },
    resources::resource::DomainResource,
};

#[derive(Debug)]
pub struct OrganizationAffiliation {
    pub resource: DomainResource,
    pub active: Option<bool>,
    pub period: Option<Period>,
    pub organization: Option<Reference>,
    pub participating_organization: Option<Reference>,
    pub network: Option<Vec<Reference>>,
    pub code: Option<Vec<CodeableConcept>>,
    pub speciality: Option<Vec<CodeableConcept>>,
    pub location: Option<Vec<Reference>>,
    pub healthcare_service: Option<Vec<Reference>>,
    pub telecom: Option<Vec<ContactPoint>>,
    pub endpoint: Option<Vec<Reference>>,
}

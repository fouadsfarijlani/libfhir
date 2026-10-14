use serde::{Deserialize, Serialize};

use crate::{
    FhirError,
    r4::{
        elements::{
            Address, Attachment, BackboneElement, CodeableConcept, ContactPoint,
            GetResourceReferences, HumanName, Identifier, Period, Reference, ReferenceTypes,
        },
        resources::{DomainResource, Organization, Practitioner, ResourceType},
    },
};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct PatientContact {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<Vec<CodeableConcept>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<HumanName>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecom: Option<Vec<ContactPoint>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Reference<Organization>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
}

impl ResourceType for PatientContact {
    const TYPE: &'static str = "PatientContact";
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct PatientCommunication {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,

    pub language: CodeableConcept,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<bool>,
}

impl ResourceType for PatientCommunication {
    const TYPE: &'static str = "PatientCommunication";
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct PatientLink {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,

    pub other: Reference<Patient>,

    pub r#type: String,
}

impl ResourceType for PatientLink {
    const TYPE: &'static str = "PatientLink";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case", serialize = "lowercase"))]
pub enum AdministrativeGender {
    Male,
    Female,
    Other,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Patient {
    #[serde(flatten)]
    pub domain_resource: DomainResource,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<HumanName>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecom: Option<Vec<ContactPoint>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<AdministrativeGender>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deceased_boolean: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deceased_date_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<Address>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<CodeableConcept>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_birth_boolean: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_birth_integer: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<Attachment>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Vec<PatientContact>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication: Option<Vec<PatientCommunication>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_practitioner: Option<Vec<Reference<Practitioner>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub managing_organization: Option<Reference<Organization>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Vec<PatientLink>>,

    #[serde(default = "Patient::get_resource_type")]
    pub resource_type: String,
}

impl Default for Patient {
    fn default() -> Self {
        Patient {
            domain_resource: DomainResource {
                ..Default::default()
            },
            resource_type: Self::get_resource_type(),
            identifier: None,
            active: None,
            name: None,
            telecom: None,
            gender: None,
            birth_date: None,
            deceased_boolean: None,
            deceased_date_time: None,
            address: None,
            marital_status: None,
            multiple_birth_boolean: None,
            multiple_birth_integer: None,
            photo: None,
            contact: None,
            communication: None,
            general_practitioner: None,
            managing_organization: None,
            link: None,
        }
    }
}

impl ResourceType for Patient {
    const TYPE: &'static str = "Patient";
}

impl Patient {
    pub fn from_json(data: &str) -> Result<Self, FhirError> {
        Ok(serde_json::from_str(data)?)
    }

    pub fn to_json_string(&self) -> Result<String, FhirError> {
        Ok(serde_json::to_string_pretty(&self)?)
    }

    pub fn to_json_value(&self) -> Result<serde_json::Value, FhirError> {
        Ok(serde_json::to_value(&self)?)
    }
}

impl GetResourceReferences for Patient {
    fn get_references(&self) -> Vec<ReferenceTypes> {
        let mut references: Vec<ReferenceTypes> = Vec::new();

        if let Some(general_practitioners) = &self.general_practitioner {
            references.extend(general_practitioners.iter().map(ReferenceTypes::from));
        }

        if let Some(managing_organization) = &self.managing_organization {
            references.push(ReferenceTypes::from(managing_organization));
        }

        if let Some(contact) = &self.contact {
            for c in contact {
                if let Some(org) = &c.organization {
                    references.push(ReferenceTypes::from(org));
                }
            }
        }

        if let Some(link) = &self.link {
            for l in link {
                references.push(ReferenceTypes::from(&l.other));
            }
        }

        references
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::r4::{
        elements::{Coding, Element, ReferenceBuilder},
        resources::{Organization, PatientBuilder, PatientContactBuilder, PatientLinkBuilder, Practitioner, Resource},
    };

    use super::*;

    #[test]
    pub fn test_from_json_should_succeed() {
        let data = include_str!("../../../../fixtures/r4/resources/patient.json");
        let expected = Patient {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("patient-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            identifier: Some(vec![
                Identifier {
                    r#use: Some("official".to_string()),
                    system: Some("http://example.org/patients".to_string()),
                    value: Some("PAT-001".to_string()),
                    ..Default::default()
                },
                Identifier {
                    r#use: Some("usual".to_string()),
                    system: Some("http://hl7.org/fhir/sid/us-ssn".to_string()),
                    value: Some("123-45-6789".to_string()),
                    ..Default::default()
                },
            ]),
            active: Some(true),
            name: Some(vec![HumanName {
                r#use: Some("official".to_string()),
                family: Some("Chalmers".to_string()),
                given: Some(vec!["Peter".to_string(), "James".to_string()]),
                ..Default::default()
            }]),
            telecom: Some(vec![
                ContactPoint {
                    system: Some("phone".to_string()),
                    value: Some("+1-555-123-4567".to_string()),
                    r#use: Some("home".to_string()),
                    ..Default::default()
                },
                ContactPoint {
                    system: Some("email".to_string()),
                    value: Some("peter.chalmers@example.org".to_string()),
                    r#use: Some("home".to_string()),
                    ..Default::default()
                },
            ]),
            gender: Some(AdministrativeGender::Male),
            birth_date: Some("1974-12-25".to_string()),
            deceased_boolean: Some(false),
            deceased_date_time: None,
            address: Some(vec![Address {
                r#use: Some("home".to_string()),
                line: Some(vec!["534 Erewhon St".to_string()]),
                city: Some("PleasantVille".to_string()),
                state: Some("Vic".to_string()),
                postal_code: Some("3999".to_string()),
                country: Some("Australia".to_string()),
                ..Default::default()
            }]),
            marital_status: Some(CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://hl7.org/fhir/ValueSet/marital-status".to_string()),
                    code: Some("M".to_string()),
                    display: Some("Married".to_string()),
                    ..Default::default()
                }]),
                text: Some("Married".to_string()),
                ..Default::default()
            }),
            multiple_birth_boolean: Some(false),
            multiple_birth_integer: None,
            photo: Some(vec![Attachment {
                content_type: Some("image/jpeg".to_string()),
                url: Some("https://example.org/photos/patient-1.jpg".to_string()),
                title: Some("Patient Photo".to_string()),
                ..Default::default()
            }]),
            contact: Some(vec![PatientContact {
                backbone_element: BackboneElement {
                    element: Element {
                        id: None,
                        extention: None,
                    },
                    modifier_extensions: None,
                },
                relationship: Some(vec![CodeableConcept {
                    coding: Some(vec![Coding {
                        system: Some(
                            "http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string(),
                        ),
                        code: Some("EMERGENCY".to_string()),
                        display: Some("Emergency Contact".to_string()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                }]),
                name: Some(HumanName {
                    family: Some("Chalmers".to_string()),
                    given: Some(vec!["Jane".to_string()]),
                    ..Default::default()
                }),
                telecom: Some(vec![ContactPoint {
                    system: Some("phone".to_string()),
                    value: Some("+1-555-987-6543".to_string()),
                    r#use: Some("mobile".to_string()),
                    ..Default::default()
                }]),
                address: None,
                gender: None,
                organization: None,
                period: None,
            }]),
            communication: Some(vec![PatientCommunication {
                backbone_element: BackboneElement {
                    element: Element {
                        id: None,
                        extention: None,
                    },
                    modifier_extensions: None,
                },
                language: CodeableConcept {
                    coding: Some(vec![Coding {
                        system: Some("urn:ietf:bcp:47".to_string()),
                        code: Some("en".to_string()),
                        display: Some("English".to_string()),
                        ..Default::default()
                    }]),
                    text: Some("English".to_string()),
                    ..Default::default()
                },
                preferred: Some(true),
            }]),
            general_practitioner: Some(vec![Reference::<Practitioner> {
                reference: Some("Practitioner/practitioner-1".to_string()),
                display: Some("Dr. John Doe".to_string()),
                ..Default::default()
            }]),
            managing_organization: Some(Reference::<Organization> {
                reference: Some("Organization/org-1".to_string()),
                display: Some("Burgers University Medical Center".to_string()),
                ..Default::default()
            }),
            link: Some(vec![PatientLink {
                backbone_element: BackboneElement {
                    element: Element {
                        id: None,
                        extention: None,
                    },
                    modifier_extensions: None,
                },
                other: Reference::<Patient> {
                    reference: Some("Patient/patient-2".to_string()),
                    r#type: Some("seealso".to_string()),
                    ..Default::default()
                },
                r#type: "seealso".to_string(),
            }]),
            ..Default::default()
        };

        let actual = Patient::from_json(data).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_to_json_value_should_succeed() {
        let expected = json!(
        {
            "resourceType": "Patient",
            "id": "some-id",
            "active": true,
            "gender": "male",
            "birthDate": "1974-12-25",
            "managingOrganization": {"reference": "Organization/1"},
            "generalPractitioner": [
                {"reference": "Practitioner/1"},
                {"reference": "Practitioner/2"}
            ]
        }
        );

        let managing_org = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();

        let gp_1 = ReferenceBuilder::default()
            .with_reference("Practitioner/1")
            .build::<Practitioner>();

        let gp_2 = ReferenceBuilder::default()
            .with_reference("Practitioner/2")
            .build::<Practitioner>();

        let general_practitioner = vec![gp_1, gp_2];
        let patient = PatientBuilder::new(String::from("some-id"))
            .active(true)
            .gender(AdministrativeGender::Male)
            .birth_date("1974-12-25")
            .managing_organization(managing_org)
            .general_practitioner(general_practitioner)
            .build();

        let actual = patient.to_json_value().unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_to_json_string_should_succeed() {
        let expected = json!(
        {
            "resourceType": "Patient",
            "id": "some-id",
            "active": true,
            "gender": "female",
            "birthDate": "1980-01-01"
        }
        );

        let patient = PatientBuilder::new(String::from("some-id"))
            .active(true)
            .gender(AdministrativeGender::Female)
            .birth_date("1980-01-01")
            .build();

        let value = patient.to_json_string().unwrap();
        let actual: serde_json::Value = serde_json::from_str(&value).unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_get_references_should_succeed() {
        let managing_org = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();
        let gp_1 = ReferenceBuilder::default()
            .with_reference("Practitioner/1")
            .build::<Practitioner>();
        let gp_2 = ReferenceBuilder::default()
            .with_reference("Practitioner/2")
            .build::<Practitioner>();
        let contact_org = ReferenceBuilder::default()
            .with_reference("Organization/contact-org")
            .build::<Organization>();
        let link_patient = ReferenceBuilder::default()
            .with_reference("Patient/linked-patient")
            .build::<Patient>();

        let expected = vec![
            ReferenceTypes::from(&gp_1),
            ReferenceTypes::from(&gp_2),
            ReferenceTypes::from(&managing_org),
            ReferenceTypes::from(&contact_org),
            ReferenceTypes::from(&link_patient),
        ];

        let contact = PatientContactBuilder::default()
            .organization(contact_org.clone())
            .build();

        let link = PatientLinkBuilder::default()
            .other(link_patient.clone())
            .r#type("seealso")
            .build();

        let patient = PatientBuilder::default()
            .managing_organization(managing_org.clone())
            .add_general_practitioner(gp_1.clone())
            .add_general_practitioner(gp_2.clone())
            .add_contact(contact)
            .add_link(link)
            .build();

        let actual = patient.get_references();

        assert_eq!(expected, actual)
    }
}

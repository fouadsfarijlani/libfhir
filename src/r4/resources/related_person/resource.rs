use serde::{Deserialize, Serialize};

use crate::{
    FhirError,
    r4::{
        elements::{
            Address, Attachment, BackboneElement, CodeableConcept, ContactPoint, GetResourceReferences,
            HumanName, Identifier, Reference, ReferenceTypes,
        },
        resources::{DomainResource, Patient, ResourceType},
    },
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct RelatedPersonCommunication {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<CodeableConcept>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<bool>,
}

impl ResourceType for RelatedPersonCommunication {
    const TYPE: &'static str = "RelatedPersonCommunication";
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct RelatedPerson {
    #[serde(flatten)]
    pub domain_resource: DomainResource,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient: Option<Reference<Patient>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<Vec<CodeableConcept>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<HumanName>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecom: Option<Vec<ContactPoint>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<Address>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<Attachment>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<crate::r4::elements::complex_types::Period>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication: Option<Vec<RelatedPersonCommunication>>,

    #[serde(default = "RelatedPerson::get_resource_type")]
    pub resource_type: String,
}

impl Default for RelatedPerson {
    fn default() -> Self {
        RelatedPerson {
            domain_resource: DomainResource {
                ..Default::default()
            },
            resource_type: Self::get_resource_type(),
            identifier: None,
            active: None,
            patient: None,
            relationship: None,
            name: None,
            telecom: None,
            gender: None,
            birth_date: None,
            address: None,
            photo: None,
            period: None,
            communication: None,
        }
    }
}

impl RelatedPerson {
    pub fn from_json(data: &str) -> Result<Self, FhirError> {
        Ok(serde_json::from_str(data)?)
    }

    pub fn to_json_value(&self) -> Result<serde_json::Value, FhirError> {
        Ok(serde_json::to_value(&self)?)
    }

    pub fn to_json_string(&self) -> Result<String, FhirError> {
        Ok(serde_json::to_string_pretty(&self)?)
    }
}

impl ResourceType for RelatedPerson {
    const TYPE: &'static str = "RelatedPerson";
}

impl GetResourceReferences for RelatedPerson {
    fn get_references(&self) -> Vec<ReferenceTypes> {
        let mut references: Vec<ReferenceTypes> = Vec::new();

        if let Some(patient) = &self.patient {
            references.push(ReferenceTypes::from(patient));
        }

        references
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::r4::{
        elements::{Coding, ReferenceBuilder},
        resources::{Patient, RelatedPersonBuilder},
    };

    use super::*;

    #[test]
    pub fn test_from_json_should_succeed() {
        let data = include_str!("../../../../fixtures/r4/resources/related_person.json");
        let result = RelatedPerson::from_json(data);
        assert!(result.is_ok());
    }

    #[test]
    pub fn test_to_json_value_should_succeed() {
        let expected = json!(
        {
            "resourceType": "RelatedPerson",
            "id": "related-person-1",
            "patient": {"reference": "Patient/patient-1"},
            "relationship": [
                {
                    "coding": [
                        {
                            "system": "http://terminology.hl7.org/CodeSystem/v3-RoleCode",
                            "code": "MTH",
                            "display": "mother"
                        }
                    ],
                    "text": "Mother"
                }
            ]
        }
        );

        let patient_ref = ReferenceBuilder::default()
            .with_reference("Patient/patient-1")
            .build::<Patient>();

        let relationship = vec![CodeableConcept {
            coding: Some(vec![Coding {
                system: Some("http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string()),
                code: Some("MTH".to_string()),
                display: Some("mother".to_string()),
                ..Default::default()
            }]),
            text: Some("Mother".to_string()),
            ..Default::default()
        }];

        let related_person = RelatedPersonBuilder::new("related-person-1")
            .patient(patient_ref)
            .relationship(relationship)
            .build();

        let actual = related_person.to_json_value().unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_to_json_string_should_succeed() {
        let patient_ref = ReferenceBuilder::default()
            .with_reference("Patient/patient-1")
            .build::<Patient>();

        let relationship = vec![CodeableConcept {
            coding: Some(vec![Coding {
                system: Some("http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string()),
                code: Some("MTH".to_string()),
                display: Some("mother".to_string()),
                ..Default::default()
            }]),
            text: Some("Mother".to_string()),
            ..Default::default()
        }];

        let related_person = RelatedPersonBuilder::new("related-person-1")
            .patient(patient_ref)
            .relationship(relationship)
            .build();

        let value = match related_person.to_json_string() {
            Ok(res) => res,
            Err(e) => panic!("{e:?}"),
        };
        let actual: serde_json::Value = serde_json::from_str(value.as_str()).unwrap();

        assert_eq!(actual["resourceType"], "RelatedPerson");
        assert_eq!(actual["id"], "related-person-1");
    }

    #[test]
    pub fn test_get_all_references_should_succeed() {
        let patient_ref = ReferenceBuilder::default()
            .with_reference("Patient/patient-1")
            .build::<Patient>();
        let expected = vec![ReferenceTypes::from(&patient_ref)];
        let related_person = RelatedPersonBuilder::default()
            .patient(patient_ref.clone())
            .build();

        let actual = related_person.get_references();

        assert_eq!(expected, actual)
    }
}

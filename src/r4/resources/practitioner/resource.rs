use serde::{Deserialize, Serialize};

use crate::{
    FhirError,
    r4::{
        elements::{
            Address, Attachement, BackboneElement, CodeableConcept, ContactPoint,
            GetResourceReferences, HumanName, Identifier, Period, Reference, ReferenceTypes,
        },
        resources::{DomainResource, Organization, ResourceType},
    },
};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct PractitionerQualification {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    pub code: CodeableConcept,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<Reference<Organization>>,
}

impl ResourceType for PractitionerQualification {
    const TYPE: &'static str = "PractitionerQualification";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case", serialize = "lowercase"))]
pub enum Gender {
    Male,
    Female,
    Other,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Practitioner {
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
    pub address: Option<Vec<Address>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>, // Date for later

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<Attachement>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification: Option<Vec<PractitionerQualification>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication: Option<Vec<CodeableConcept>>,

    pub resource_type: String,
}

impl Default for Practitioner {
    fn default() -> Self {
        Practitioner {
            domain_resource: DomainResource {
                ..Default::default()
            },
            resource_type: Self::get_resource_type(),
            identifier: None,
            active: None,
            name: None,
            telecom: None,
            address: None,
            gender: None,
            birth_date: None,
            photo: None,
            qualification: None,
            communication: None,
        }
    }
}

impl ResourceType for Practitioner {
    const TYPE: &'static str = "Practitioner";
}

impl Practitioner {
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

impl GetResourceReferences for Practitioner {
    fn get_references(&self) -> Vec<ReferenceTypes> {
        let mut references = Vec::<ReferenceTypes>::new();

        if let Some(qualifications) = &self.qualification {
            for q in qualifications {
                if let Some(issuer) = &q.issuer {
                    references.push(ReferenceTypes::from(issuer));
                }
            }
        }

        references
    }
}

#[cfg(test)]
mod test {

    use serde_json::json;

    use crate::r4::{
        elements::{Coding, ReferenceBuilder},
        resources::{PractitionerBuilder, PractitionerQualificationBuilder, Resource},
    };

    use super::*;

    #[test]
    pub fn get_practitoner_from_json() {
        let data = include_str!("../../../../fixtures/r4/resources/practitioner.json");
        let practitioner = Practitioner::from_json(data);

        print!("{:#?}", practitioner)
    }

    #[test]
    pub fn test_from_json_should_succeed() {
        let data = include_str!("../../../../fixtures/r4/resources/practitioner.json");
        let expected = Practitioner {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("practitioner-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            identifier: Some(vec![Identifier {
                r#use: Some("official".to_string()),
                system: Some("http://example.org/practitioners".to_string()),
                value: Some("PRAC-001".to_string()),
                ..Default::default()
            }]),
            active: Some(true),
            name: Some(vec![HumanName {
                r#use: Some("official".to_string()),
                family: Some("Doe".to_string()),
                given: Some(vec!["John".to_string()]),
                prefix: Some(vec!["Dr".to_string()]),
                ..Default::default()
            }]),
            telecom: Some(vec![
                ContactPoint {
                    system: Some("phone".to_string()),
                    value: Some("+1-555-777-8888".to_string()),
                    r#use: Some("work".to_string()),
                    ..Default::default()
                },
                ContactPoint {
                    system: Some("email".to_string()),
                    value: Some("john.doe@bumc.example.org".to_string()),
                    r#use: Some("work".to_string()),
                    ..Default::default()
                },
            ]),
            address: Some(vec![Address {
                r#use: Some("work".to_string()),
                line: Some(vec!["456 Health St".to_string()]),
                city: Some("PleasantVille".to_string()),
                state: Some("Vic".to_string()),
                postal_code: Some("3999".to_string()),
                country: Some("Australia".to_string()),
                ..Default::default()
            }]),
            gender: Some(Gender::Male),
            birth_date: Some("1980-04-15".to_string()),
            photo: Some(vec![Attachement {
                content_type: Some("image/jpeg".to_string()),
                url: Some("https://example.org/photos/practitioner-1.jpg".to_string()),
                title: Some("Profile photo".to_string()),
                ..Default::default()
            }]),
            qualification: Some(vec![PractitionerQualification {
                identifier: Some(vec![Identifier {
                    system: Some("http://example.org/licenses".to_string()),
                    value: Some("LIC-123456".to_string()),
                    ..Default::default()
                }]),
                code: CodeableConcept {
                    coding: Some(vec![Coding {
                        system: Some("http://terminology.hl7.org/CodeSystem/v2-0360".to_string()),
                        code: Some("MD".to_string()),
                        display: Some("Doctor of Medicine".to_string()),
                        ..Default::default()
                    }]),
                    text: Some("MD".to_string()),
                    ..Default::default()
                },
                period: Some(Period {
                    start: Some("2010-01-01".to_string()),
                    ..Default::default()
                }),
                issuer: Some(Reference::<Organization> {
                    reference: Some("Organization/org-licensing-board".to_string()),
                    display: Some("Medical Licensing Board".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }]),
            communication: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("urn:ietf:bcp:47".to_string()),
                    code: Some("en".to_string()),
                    display: Some("English".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let actual = Practitioner::from_json(data).unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_to_json_string_should_succeed() {
        let expected = json!({
            "resourceType": "Practitioner",
            "id": "prac-1",
            "gender": "female"
        });

        let practitioner = Practitioner {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("prac-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            gender: Some(Gender::Female),
            ..Default::default()
        };

        let value = practitioner
            .to_json_string()
            .unwrap_or_else(|e| panic!("{e:?}"));

        let actual: serde_json::Value = serde_json::from_str(&value.as_str()).unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_to_json_value_should_succeed() {
        let expected = json!({
            "resourceType": "Practitioner",
            "id": "prac-1",
            "gender": "female"
        });
        let practitioner = Practitioner {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("prac-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            gender: Some(Gender::Female),
            ..Default::default()
        };

        let actual = practitioner
            .to_json_value()
            .unwrap_or_else(|e| panic!("{e:?}"));

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_get_references_should_succeed() {
        let issuer_1 = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();
        let issuer_2 = ReferenceBuilder::default()
            .with_reference("Organization/2")
            .build::<Organization>();
        let qualification_1 = PractitionerQualificationBuilder::default()
            .issuer(issuer_1.clone())
            .build();
        let qualification_2 = PractitionerQualificationBuilder::default()
            .issuer(issuer_2.clone())
            .build();
        let practitioner = PractitionerBuilder::default()
            .qualification(vec![qualification_1, qualification_2])
            .build();
        let expected = vec![
            ReferenceTypes::from(&issuer_1),
            ReferenceTypes::from(&issuer_2),
        ];

        let actual = practitioner.get_references();

        assert_eq!(expected, actual)
    }
}

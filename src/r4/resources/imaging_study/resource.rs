use serde::{Deserialize, Serialize};

use crate::{
    FhirError,
    r4::{
        elements::{
            BackboneElement, CodeableConcept, GetResourceReferences, Identifier, Reference,
            ReferenceTypes,
        },
        resources::{DomainResource, Endpoint, Patient, Practitioner, ResourceType},
    },
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum ImagingStudyStatus {
    Registered,
    Available,
    Cancelled,
    EnteredInError,
    Unknown,
}

impl Default for ImagingStudyStatus {
    fn default() -> Self {
        ImagingStudyStatus::Unknown
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImagingStudyInstance {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,

    pub uid: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sopClass")]
    pub sop_class: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ResourceType for ImagingStudyInstance {
    const TYPE: &'static str = "ImagingStudyInstance";
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImagingStudySeries {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,

    pub uid: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,

    pub modality: CodeableConcept,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "numberOfInstances")]
    pub number_of_instances: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Vec<Reference<Endpoint>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableConcept>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "laterality")]
    pub laterality: Option<CodeableConcept>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<Vec<ImagingStudyInstance>>,
}

impl ResourceType for ImagingStudySeries {
    const TYPE: &'static str = "ImagingStudySeries";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct ImagingStudy {
    #[serde(flatten)]
    pub domain_resource: DomainResource,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ImagingStudyStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modality: Option<Vec<CodeableConcept>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CodeableConcept>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Reference<Patient>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub based_on: Option<Vec<Reference<Practitioner>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<Reference<Practitioner>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpreter: Option<Vec<Reference<Practitioner>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Vec<Reference<Endpoint>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "numberOfSeries")]
    pub number_of_series: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "numberOfInstances")]
    pub number_of_instances: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<ImagingStudySeries>>,

    #[serde(default = "ImagingStudy::get_resource_type")]
    pub resource_type: String,
}

impl Default for ImagingStudy {
    fn default() -> Self {
        ImagingStudy {
            domain_resource: DomainResource {
                ..Default::default()
            },
            resource_type: Self::get_resource_type(),
            identifier: None,
            status: None,
            modality: None,
            r#type: None,
            subject: None,
            started: None,
            based_on: None,
            referrer: None,
            interpreter: None,
            endpoint: None,
            number_of_series: None,
            number_of_instances: None,
            series: None,
        }
    }
}

impl ResourceType for ImagingStudy {
    const TYPE: &'static str = "ImagingStudy";
}

impl ImagingStudy {
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

impl GetResourceReferences for ImagingStudy {
    fn get_references(&self) -> Vec<ReferenceTypes> {
        let mut references: Vec<ReferenceTypes> = Vec::new();

        if let Some(subject) = &self.subject {
            references.push(ReferenceTypes::from(subject));
        }

        if let Some(based_on) = &self.based_on {
            references.extend(based_on.iter().map(ReferenceTypes::from));
        }

        if let Some(referrer) = &self.referrer {
            references.push(ReferenceTypes::from(referrer));
        }

        if let Some(interpreter) = &self.interpreter {
            references.extend(interpreter.iter().map(ReferenceTypes::from));
        }

        if let Some(endpoint) = &self.endpoint {
            references.extend(endpoint.iter().map(ReferenceTypes::from));
        }

        if let Some(series) = &self.series {
            for s in series {
                if let Some(endpoints) = &s.endpoint {
                    references.extend(endpoints.iter().map(ReferenceTypes::from));
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
        elements::{Coding, Element, ReferenceBuilder},
        resources::{Endpoint, ImagingStudyBuilder, Patient, Practitioner, Resource},
    };

    use super::*;

    #[test]
    pub fn test_from_json_should_succeed() {
        let data = include_str!("../../../../fixtures/r4/resources/imaging_study.json");
        let expected = ImagingStudy {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("imaging-study-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            identifier: Some(vec![
                Identifier {
                    r#use: Some("official".to_string()),
                    system: Some("urn:dicom:uid".to_string()),
                    value: Some("urn:oid:1.2.840.113619.2.55.3".to_string()),
                    ..Default::default()
                },
                Identifier {
                    r#use: Some("secondary".to_string()),
                    system: Some("http://hospital.example.org/study-ids".to_string()),
                    value: Some("STUDY-12345".to_string()),
                    ..Default::default()
                },
            ]),
            status: Some(ImagingStudyStatus::Available),
            modality: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://dicom.nema.org/resources/ontology/DCM".to_string()),
                    code: Some("CT".to_string()),
                    display: Some("Computed Tomography".to_string()),
                    ..Default::default()
                }]),
                text: Some("CT".to_string()),
                ..Default::default()
            }]),
            r#type: Some(CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://loinc.org".to_string()),
                    code: Some("24725-8".to_string()),
                    display: Some("CT of head".to_string()),
                    ..Default::default()
                }]),
                text: Some("CT of head without contrast".to_string()),
                ..Default::default()
            }),
            subject: Some(Reference::<Patient> {
                reference: Some("Patient/patient-1".to_string()),
                display: Some("Peter Chalmers".to_string()),
                ..Default::default()
            }),
            started: Some("2023-01-15T10:30:00Z".to_string()),
            based_on: Some(vec![Reference::<Practitioner> {
                reference: Some("Practitioner/practitioner-1".to_string()),
                display: Some("Dr. John Doe".to_string()),
                ..Default::default()
            }]),
            referrer: Some(Reference::<Practitioner> {
                reference: Some("Practitioner/practitioner-2".to_string()),
                display: Some("Dr. Jane Smith".to_string()),
                ..Default::default()
            }),
            interpreter: Some(vec![Reference::<Practitioner> {
                reference: Some("Practitioner/practitioner-3".to_string()),
                display: Some("Dr. Robert Johnson".to_string()),
                ..Default::default()
            }]),
            endpoint: Some(vec![Reference::<Endpoint> {
                reference: Some("Endpoint/endpoint-1".to_string()),
                display: Some("PACS Server".to_string()),
                ..Default::default()
            }]),
            number_of_series: Some(1),
            number_of_instances: Some(10),
            series: Some(vec![ImagingStudySeries {
                backbone_element: BackboneElement {
                    element: Element {
                        id: None,
                        extention: None,
                    },
                    modifier_extensions: None,
                },
                uid: "urn:oid:1.2.840.113619.2.55.3.123456789".to_string(),
                number: Some(1),
                modality: CodeableConcept {
                    coding: Some(vec![Coding {
                        system: Some("http://dicom.nema.org/resources/ontology/DCM".to_string()),
                        code: Some("CT".to_string()),
                        display: Some("Computed Tomography".to_string()),
                        ..Default::default()
                    }]),
                    text: Some("CT".to_string()),
                    ..Default::default()
                },
                description: Some("CT Head without contrast".to_string()),
                number_of_instances: Some(10),
                body_site: Some(CodeableConcept {
                    coding: Some(vec![Coding {
                        system: Some("http://snomed.info/sct".to_string()),
                        code: Some("27267000".to_string()),
                        display: Some("Head".to_string()),
                        ..Default::default()
                    }]),
                    text: Some("Head".to_string()),
                    ..Default::default()
                }),
                laterality: Some(CodeableConcept {
                    coding: Some(vec![Coding {
                        system: Some("http://snomed.info/sct".to_string()),
                        code: Some("51440002".to_string()),
                        display: Some("Bilateral".to_string()),
                        ..Default::default()
                    }]),
                    text: Some("Bilateral".to_string()),
                    ..Default::default()
                }),
                started: Some("2023-01-15T10:35:00Z".to_string()),
                endpoint: Some(vec![Reference::<Endpoint> {
                    reference: Some("Endpoint/endpoint-2".to_string()),
                    display: Some("DICOM Archive".to_string()),
                    ..Default::default()
                }]),
                instance: Some(vec![
                    ImagingStudyInstance {
                        backbone_element: BackboneElement {
                            element: Element {
                                id: None,
                                extention: None,
                            },
                            modifier_extensions: None,
                        },
                        uid: "urn:oid:1.2.840.113619.2.55.3.123456789.1".to_string(),
                        number: Some(1),
                        sop_class: Some("urn:oid:1.2.840.10008.5.1.4.1.1.2".to_string()),
                        title: Some("CT Image 1 - Axial".to_string()),
                    },
                    ImagingStudyInstance {
                        backbone_element: BackboneElement {
                            element: Element {
                                id: None,
                                extention: None,
                            },
                            modifier_extensions: None,
                        },
                        uid: "urn:oid:1.2.840.113619.2.55.3.123456789.2".to_string(),
                        number: Some(2),
                        sop_class: Some("urn:oid:1.2.840.10008.5.1.4.1.1.2".to_string()),
                        title: Some("CT Image 2 - Axial".to_string()),
                    },
                ]),
            }]),
            ..Default::default()
        };

        let actual = ImagingStudy::from_json(data).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_to_json_value_should_succeed() {
        let expected = json!({
            "resourceType": "ImagingStudy",
            "id": "study-1",
            "status": "available",
            "numberOfSeries": 1,
            "numberOfInstances": 10
        });

        let study = ImagingStudyBuilder::new("study-1")
            .status(ImagingStudyStatus::Available)
            .number_of_series(1)
            .number_of_instances(10)
            .build();

        let actual = study.to_json_value().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_to_json_string_should_succeed() {
        let expected = json!({
            "resourceType": "ImagingStudy",
            "id": "study-1",
            "status": "registered"
        });

        let study = ImagingStudyBuilder::new("study-1")
            .status(ImagingStudyStatus::Registered)
            .build();

        let value = study.to_json_string().unwrap();
        let actual: serde_json::Value = serde_json::from_str(&value).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_get_references_should_succeed() {
        let subject = ReferenceBuilder::default()
            .with_reference("Patient/1")
            .build::<Patient>();
        let based_on = ReferenceBuilder::default()
            .with_reference("Practitioner/1")
            .build::<Practitioner>();
        let referrer = ReferenceBuilder::default()
            .with_reference("Practitioner/2")
            .build::<Practitioner>();
        let interpreter = ReferenceBuilder::default()
            .with_reference("Practitioner/3")
            .build::<Practitioner>();
        let endpoint = ReferenceBuilder::default()
            .with_reference("Endpoint/1")
            .build::<Endpoint>();

        let expected = vec![
            ReferenceTypes::from(&subject),
            ReferenceTypes::from(&based_on),
            ReferenceTypes::from(&referrer),
            ReferenceTypes::from(&interpreter),
            ReferenceTypes::from(&endpoint),
        ];

        let study = ImagingStudyBuilder::default()
            .subject(subject.clone())
            .add_based_on(based_on.clone())
            .referrer(referrer.clone())
            .add_interpreter(interpreter.clone())
            .add_endpoint(endpoint.clone())
            .build();

        let actual = study.get_references();

        assert_eq!(expected, actual);
    }
}

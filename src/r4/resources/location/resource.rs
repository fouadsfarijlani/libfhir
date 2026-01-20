use serde::{Deserialize, Serialize};

use crate::{
    FhirError,
    r4::{
        elements::{
            Address, BackboneElement, CodeableConcept, Coding, ContactPoint, DaysOfWeek,
            GetResourceReferences, Identifier, Reference, ReferenceTypes,
        },
        resources::{self, DomainResource, Endpoint, Organization, ResourceType},
    },
};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct LocationPosition {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,
    pub longitude: f32,
    pub latitude: f32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<f32>,
}

impl ResourceType for LocationPosition {
    const TYPE: &'static str = "LocationPosition";
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct HoursOfOperation {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_of_week: Option<Vec<DaysOfWeek>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_day: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_time: Option<String>, // to be resolved later

    #[serde(skip_serializing_if = "Option::is_none")]
    pub closing_time: Option<String>, // to be resolved later
}

impl ResourceType for HoursOfOperation {
    const TYPE: &'static str = "HoursOfOperation";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case", serialize = "lowercase"))]
pub enum LocationStatus {
    Active,
    Suspended,
    Inactive,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Location {
    #[serde(flatten)]
    pub domain_resource: DomainResource,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LocationStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<Coding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<CodeableConcept>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecom: Option<Vec<ContactPoint>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_type: Option<CodeableConcept>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<LocationPosition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub managing_organization: Option<Reference<Organization>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of: Option<Reference<Location>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation: Option<Vec<HoursOfOperation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_exceptions: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Reference<Endpoint>>,

    #[serde(default = "Location::get_resource_type")]
    pub resource_type: String,
}

impl ResourceType for Location {
    const TYPE: &'static str = "Location";
}

impl Default for Location {
    fn default() -> Self {
        Location {
            resource_type: Self::get_resource_type(),
            domain_resource: DomainResource {
                ..Default::default()
            },
            identifier: None,
            status: None,
            operational_status: None,
            name: None,
            alias: None,
            description: None,
            mode: None,
            r#type: None,
            telecom: None,
            address: None,
            physical_type: None,
            position: None,
            part_of: None,
            hours_of_operation: None,
            availability_exceptions: None,
            endpoint: None,
            managing_organization: None,
        }
    }
}

impl Location {
    pub fn from_json(data: &str) -> Self {
        resources::from_json(data)
    }

    pub fn to_json_value(&self) -> Result<serde_json::Value, FhirError> {
        Ok(serde_json::to_value(&self)?)
    }

    pub fn to_json_string(&self) -> Result<String, FhirError> {
        Ok(serde_json::to_string_pretty(&self)?)
    }
}

impl GetResourceReferences for Location {
    fn get_references(&self) -> Vec<ReferenceTypes> {
        let mut references = Vec::<ReferenceTypes>::new();

        if let Some(man_org) = &self.managing_organization {
            references.push(ReferenceTypes::from(man_org));
        }

        if let Some(part_of) = &self.part_of {
            references.push(ReferenceTypes::from(part_of));
        }

        if let Some(endpoint) = &self.endpoint {
            references.push(ReferenceTypes::from(endpoint));
        }

        references
    }
}

#[cfg(test)]
mod test {

    use crate::r4::{
        elements::ReferenceBuilder,
        resources::{LocationBuilder, Resource},
    };
    use serde_json::json;

    use super::*;

    #[test]
    pub fn test_from_json_should_succeed() {
        let data = include_str!("../../../../fixtures/r4/resources/location.json");
        let expected = Location {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("location-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            identifier: Some(vec![Identifier {
                system: Some("http://example.com/locations".to_string()),
                value: Some("loc-001".to_string()),
                ..Default::default()
            }]),
            status: Some(LocationStatus::Active),
            operational_status: Some(Coding {
                system: Some("http://terminology.hl7.org/CodeSystem/v2-0116".to_string()),
                code: Some("C".to_string()),
                display: Some("Closed".to_string()),
                ..Default::default()
            }),
            name: Some("South Wing, second floor".to_string()),
            alias: Some(vec!["SW-2".to_string()]),
            description: Some("Second floor of the south wing".to_string()),
            mode: Some("instance".to_string()),
            r#type: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string()),
                    code: Some("HOSP".to_string()),
                    display: Some("Hospital".to_string()),
                    ..Default::default()
                }]),
                text: Some("Hospital".to_string()),
                ..Default::default()
            }]),
            telecom: Some(vec![ContactPoint {
                system: Some("phone".to_string()),
                value: Some("2328".to_string()),
                ..Default::default()
            }]),
            address: Some(Address {
                r#use: Some("work".to_string()),
                line: Some(vec!["South Wing, floor 2".to_string()]),
                city: Some("PleasantVille".to_string()),
                state: Some("Vic".to_string()),
                postal_code: Some("3999".to_string()),
                country: Some("Australia".to_string()),
                ..Default::default()
            }),
            physical_type: Some(CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some(
                        "http://terminology.hl7.org/CodeSystem/location-physical-type".to_string(),
                    ),
                    code: Some("wi".to_string()),
                    display: Some("Wing".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }),
            position: Some(LocationPosition {
                backbone_element: BackboneElement::default(),
                longitude: -83.69,
                latitude: 42.25,
                altitude: Some(0.0),
            }),
            hours_of_operation: Some(vec![HoursOfOperation {
                backbone_element: BackboneElement::default(),
                days_of_week: Some(vec![DaysOfWeek::Mon, DaysOfWeek::Tue, DaysOfWeek::Wed]),
                all_day: Some(false),
                operation_time: None,
                closing_time: None,
            }]),
            availability_exceptions: Some("Closed on public holidays".to_string()),
            managing_organization: Some(Reference {
                reference: Some("Organization/1".to_string()),
                display: Some("Burgers University Medical Center".to_string()),
                ..Default::default()
            }),
            part_of: Some(Reference {
                reference: Some("Location/parent".to_string()),
                ..Default::default()
            }),
            endpoint: Some(Reference {
                reference: Some("Endpoint/1".to_string()),
                ..Default::default()
            }),
            resource_type: "Location".to_string(),
        };

        let actual = Location::from_json(data);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_get_references_should_succeed() {
        let managing_org_ref = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();

        let part_of_ref = ReferenceBuilder::default()
            .with_reference("Location/2")
            .build::<Location>();

        let endpoint_ref = ReferenceBuilder::default()
            .with_reference("Endpoint/1")
            .build::<Endpoint>();

        let expected = vec![
            ReferenceTypes::from(&managing_org_ref),
            ReferenceTypes::from(&part_of_ref),
            ReferenceTypes::from(&endpoint_ref),
        ];

        let location = LocationBuilder::default()
            .endpoint(endpoint_ref.clone())
            .managing_organization(managing_org_ref.clone())
            .part_of(part_of_ref.clone())
            .build();

        let actual = location.get_references();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_to_json_value_should_succeed() {
        let managing_org = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();

        let location = LocationBuilder::new("location-1")
            .status(LocationStatus::Active)
            .managing_organization(managing_org)
            .build();

        let expected = json!({
            "resourceType": "Location",
            "id": "location-1",
            "status": "active",
            "managingOrganization": {
                "reference": "Organization/1"
            }
        });

        let actual = location.to_json_value().unwrap_or_else(|e| panic!("{e:?}"));

        assert_eq!(expected, actual);
    }
}

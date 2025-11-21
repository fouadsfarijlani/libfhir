use std::vec;

use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        Address, BackboneElement, CodeableConcept, Coding, ContactPoint, GetResourceReferences,
        Identifier, Reference, ReferenceTypes,
    },
    resources::{self, DomainResource, Endpoint, Organization, ResourceType},
};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct LocationPosition {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,
    pub longitude: f32,
    pub latitude: f32,
    pub altitude: Option<f32>,
}

impl ResourceType for LocationPosition {
    const TYPE: &'static str = "LocationPosition";
}

#[derive(Default)]
pub struct LocationPositionBuilder {
    backbone_element: BackboneElement,
    longitude: f32,
    latitude: f32,
    altitude: Option<f32>,
}

impl LocationPositionBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.backbone_element.element.id = Some(id.into());
        builder
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.backbone_element.element.id = Some(id.into());
        self
    }

    pub fn with_longitude(mut self, longitude: f32) -> Self {
        self.longitude = longitude;
        self
    }

    pub fn with_latitude(mut self, latitude: f32) -> Self {
        self.latitude = latitude;
        self
    }

    pub fn with_altitude(mut self, altitude: f32) -> Self {
        self.altitude = Some(altitude);
        self
    }

    pub fn build(self) -> LocationPosition {
        LocationPosition {
            backbone_element: self.backbone_element,
            longitude: self.longitude,
            latitude: self.latitude,
            altitude: self.altitude,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct HoursOfOperation {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,
    pub days_of_week: Option<Vec<String>>,
    pub all_day: Option<bool>,
    pub operation_time: Option<String>, // to be resolved later
    pub closing_time: Option<String>,   // to be resolved later
}

impl ResourceType for HoursOfOperation {
    const TYPE: &'static str = "HoursOfOperation";
}

#[derive(Default)]
pub struct HoursOfOperationBuilder {
    backbone_element: BackboneElement,
    days_of_week: Option<Vec<String>>,
    all_day: Option<bool>,
    operation_time: Option<String>, // to be resolved later
    closing_time: Option<String>,   // to be resolved later
}

impl HoursOfOperationBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.backbone_element.element.id = Some(id.into());
        builder
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.backbone_element.element.id = Some(id.into());
        self
    }
    pub fn with_days_of_week(mut self, days: Vec<String>) -> Self {
        self.days_of_week = Some(days);
        self
    }

    pub fn add_day_of_week(mut self, day: impl Into<String>) -> Self {
        match &mut self.days_of_week {
            Some(dow) => dow.push(day.into()),
            None => self.days_of_week = Some(vec![day.into()]),
        }
        self
    }

    pub fn with_all_day(mut self, value: bool) -> Self {
        self.all_day = Some(value);
        self
    }

    pub fn with_operation_time(mut self, time: impl Into<String>) -> Self {
        self.operation_time = Some(time.into());
        self
    }

    pub fn with_closing_time(mut self, time: impl Into<String>) -> Self {
        self.closing_time = Some(time.into());
        self
    }

    pub fn build(self) -> HoursOfOperation {
        HoursOfOperation {
            backbone_element: self.backbone_element,
            days_of_week: self.days_of_week,
            all_day: self.all_day,
            operation_time: self.operation_time,
            closing_time: self.closing_time,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Location {
    #[serde(flatten)]
    pub domain_resource: DomainResource,
    pub identifier: Option<Vec<Identifier>>,
    pub status: Option<String>,
    pub operational_status: Option<Coding>,
    pub name: Option<String>,
    pub alias: Option<Vec<String>>,
    pub description: Option<String>,
    pub mode: Option<String>,
    pub r#type: Option<Vec<CodeableConcept>>,
    pub telecom: Option<Vec<ContactPoint>>,
    pub address: Option<Address>,
    pub physical_type: Option<CodeableConcept>,
    pub position: Option<LocationPosition>,
    pub managing_organization: Option<Reference<Organization>>,
    pub part_of: Option<Reference<Location>>,
    pub hours_of_operation: Option<Vec<HoursOfOperation>>,
    pub availability_exceptions: Option<String>,
    pub endpoint: Option<Reference<Endpoint>>,
}

impl ResourceType for Location {
    const TYPE: &'static str = "Location";
}

impl Location {
    pub fn from_json(data: &str) -> Self {
        resources::from_json(data)
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

#[derive(Default)]
pub struct LocationBuilder {
    domain_resource: DomainResource,
    identifier: Option<Vec<Identifier>>,
    status: Option<String>,
    operational_status: Option<Coding>,
    name: Option<String>,
    alias: Option<Vec<String>>,
    description: Option<String>,
    mode: Option<String>,
    r#type: Option<Vec<CodeableConcept>>,
    telecom: Option<Vec<ContactPoint>>,
    address: Option<Address>,
    physical_type: Option<CodeableConcept>,
    position: Option<LocationPosition>,
    managing_organization: Option<Reference<Organization>>,
    part_of: Option<Reference<Location>>,
    hours_of_operation: Option<Vec<HoursOfOperation>>,
    availability_exceptions: Option<String>,
    endpoint: Option<Reference<Endpoint>>,
}

impl LocationBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.domain_resource.resource.id = Some(id.into());
        builder
    }

    pub fn with_identifier(mut self, identifier: Vec<Identifier>) -> Self {
        self.identifier = Some(identifier);
        self
    }

    pub fn add_identifier(mut self, identifier: Identifier) -> Self {
        match &mut self.identifier {
            Some(ident) => ident.push(identifier),
            None => self.identifier = Some(vec![identifier]),
        }
        self
    }
    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn with_operational_status(mut self, coding: Coding) -> Self {
        self.operational_status = Some(coding);
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_alias(mut self, aliases: Vec<String>) -> Self {
        self.alias = Some(aliases);
        self
    }

    pub fn add_alias(mut self, alias: impl Into<String>) -> Self {
        match &mut self.alias {
            Some(a) => a.push(alias.into()),
            None => self.alias = Some(vec![alias.into()]),
        }
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_mode(mut self, mode: impl Into<String>) -> Self {
        self.mode = Some(mode.into());
        self
    }

    pub fn with_type(mut self, r#type: Vec<CodeableConcept>) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn add_type(mut self, r#type: CodeableConcept) -> Self {
        match &mut self.r#type {
            Some(t) => t.push(r#type),
            None => self.r#type = Some(vec![r#type]),
        }
        self
    }

    pub fn with_telecom(mut self, telecom: Vec<ContactPoint>) -> Self {
        self.telecom = Some(telecom);
        self
    }

    pub fn add_telecom(mut self, telecom: ContactPoint) -> Self {
        match &mut self.telecom {
            Some(t) => t.push(telecom),
            None => self.telecom = Some(vec![telecom]),
        }
        self
    }

    pub fn with_address(mut self, address: Address) -> Self {
        self.address = Some(address);
        self
    }

    pub fn with_physical_type(mut self, concept: CodeableConcept) -> Self {
        self.physical_type = Some(concept);
        self
    }

    pub fn with_position(mut self, position: LocationPosition) -> Self {
        self.position = Some(position);
        self
    }

    pub fn with_managing_organization(mut self, org: Reference<Organization>) -> Self {
        self.managing_organization = Some(org);
        self
    }

    pub fn with_part_of(mut self, part_of: Reference<Location>) -> Self {
        self.part_of = Some(part_of);
        self
    }

    pub fn with_hours_of_operation(mut self, hours: Vec<HoursOfOperation>) -> Self {
        self.hours_of_operation = Some(hours);
        self
    }

    pub fn add_hours_of_operation(mut self, hours: HoursOfOperation) -> Self {
        match &mut self.hours_of_operation {
            Some(hop) => hop.push(hours),
            None => self.hours_of_operation = Some(vec![hours]),
        }
        self
    }

    pub fn with_availability_exceptions(mut self, text: impl Into<String>) -> Self {
        self.availability_exceptions = Some(text.into());
        self
    }

    pub fn with_endpoint(mut self, endpoint: Reference<Endpoint>) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    pub fn build(self) -> Location {
        Location {
            domain_resource: self.domain_resource,
            identifier: self.identifier,
            status: self.status,
            operational_status: self.operational_status,
            name: self.name,
            alias: self.alias,
            description: self.description,
            mode: self.mode,
            r#type: self.r#type,
            telecom: self.telecom,
            address: self.address,
            physical_type: self.physical_type,
            position: self.position,
            managing_organization: self.managing_organization,
            part_of: self.part_of,
            hours_of_operation: self.hours_of_operation,
            availability_exceptions: self.availability_exceptions,
            endpoint: self.endpoint,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        elements::{
            AddressBuilder, CodeableConceptBuilder, CodingBuilder, ContactPointBuilder, Element,
            IdentifierBuilder, ReferenceBuilder,
        },
        resources::Resource,
    };

    use super::*;

    #[test]
    fn test_build_location_position_should_succeed() {
        let mut expected = LocationPosition::default();
        expected.backbone_element.element.id = Some("position-1".to_string());
        expected.latitude = 10.00;
        expected.longitude = 20.00;
        expected.altitude = Some(20.00);

        let actual = LocationPositionBuilder::default()
            .with_id("position-1")
            .with_latitude(10.00)
            .with_longitude(20.00)
            .with_altitude(20.00)
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_build_hours_of_operation_should_succeed() {
        let mut expected = HoursOfOperation::default();
        expected.days_of_week = Some(vec!["Monday".to_string(), "Thursday".to_string()]);
        expected.all_day = Some(true);
        expected.operation_time = Some("all day".to_string());
        expected.closing_time = Some("midnight".to_string());

        let actual = HoursOfOperationBuilder::default()
            .add_day_of_week("Monday")
            .add_day_of_week("Thursday")
            .with_all_day(true)
            .with_operation_time("all day")
            .with_closing_time("midnight")
            .build();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_from_json_should_succeed() {
        let data = r#"
            {
                "resourceType": "Location",
                "id": "location-1",
                "status": "active",
                "identifier": [
                    {
                        "system": "http://example.com",
                        "value": "identifier-1"
                    }
                ],
                "name": "South Wing, second floor",
                "description": "Second floor",
                "mode": "instance",
                "type": [
                            {
                                "coding": [
                                            {
                                                "system": "http://example.com",
                                                "code": "HOSP",
                                                "display": "Hospital"
                                            }
                                        ],
                                "text": "Hospital"
                            }
                        ],
                "telecom": [
                            {
                                "system": "phone",
                                "value": "2328"
                            }
                            ],
                "address": {
                                "use": "work",
                                "line": ["South Wing, floor 2"],
                                "city": "PleasantVille",
                                "state": "Vic",
                                "postalCode": "3999",
                                "country": "Australia"
                            },
                "physicalType": {
                                "coding": [
                                            {
                                                "system": "http://example.com",
                                                "code": "wi",
                                                "display": "Wing"
                                            }
                                        ]
                                },
                "position": {
                                "longitude": -83.69,
                                "latitude": 42.25,
                                "altitude": 0.0
                            },
                "managingOrganization": {
                        "reference": "Organization/1",
                        "display": "Burgers University Medical Center"
                        }
            }
            "#;

        let identifier = IdentifierBuilder::default()
            .with_system("http://example.com")
            .with_value("identifier-1")
            .build();
        let location_coding = CodingBuilder::default()
            .with_code("HOSP")
            .with_system("http://example.com")
            .with_display("Hospital")
            .build();
        let location_type = CodeableConceptBuilder::default()
            .add_coding(location_coding)
            .with_text("Hospital")
            .build();
        let telecom = ContactPointBuilder::default()
            .with_system("phone")
            .with_value("2328")
            .build();
        let address = AddressBuilder::default()
            .with_use("work")
            .add_line("South Wing, floor 2")
            .with_city("PleasantVille")
            .with_state("Vic")
            .with_postal_code("3999")
            .with_country("Australia")
            .build();
        let physical_type_coding = CodingBuilder::default()
            .with_system("http://example.com")
            .with_code("wi")
            .with_display("Wing")
            .build();
        let physical_type = CodeableConceptBuilder::default()
            .add_coding(physical_type_coding)
            .build();
        let position = LocationPositionBuilder::default()
            .with_longitude(-83.69)
            .with_latitude(42.25)
            .with_altitude(0.0)
            .build();
        let managing_organization = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .with_display("Burgers University Medical Center")
            .build::<Organization>();
        let expected = LocationBuilder::new("location-1")
            .with_status("active")
            .add_identifier(identifier)
            .with_name("South Wing, second floor")
            .with_description("Second floor")
            .with_physical_type(physical_type)
            .with_mode("instance")
            .add_type(location_type)
            .add_telecom(telecom)
            .with_address(address)
            .with_position(position)
            .with_managing_organization(managing_organization)
            .build();

        let actual = Location::from_json(data);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_build_location_shoud_succeed() {
        let expected = Location {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("location-1".to_string()),
                    implicit_rules: None,
                    meta: None,
                },
                text: None,
                contained: None,
                exnetions: None,
            },
            alias: Some(vec!["alias".to_string()]),
            address: Some(Address {
                element: Element {
                    id: None,
                    extention: None,
                },
                r#use: Some("official".to_string()),
                r#type: None,
                text: None,
                city: None,
                line: None,
                district: None,
                state: Some("Arizona".to_string()),
                period: None,
                postal_code: None,
                country: None,
            }),
            availability_exceptions: None,
            description: Some("text".to_string()),
            identifier: None,
            status: Some("active".to_string()),
            hours_of_operation: None,
            managing_organization: None,
            endpoint: None,
            operational_status: None,
            part_of: None,
            mode: None,
            name: None,
            physical_type: None,
            r#type: None,
            telecom: None,
            position: None,
        };
        let actual_address = Address {
            element: Element {
                id: None,
                extention: None,
            },
            r#use: Some("official".to_string()),
            r#type: None,
            text: None,
            city: None,
            line: None,
            district: None,
            state: Some("Arizona".to_string()),
            period: None,
            postal_code: None,
            country: None,
        };
        let actual = LocationBuilder::new("location-1")
            .with_alias(vec!["alias".to_string()])
            .with_address(actual_address)
            .with_description("text")
            .with_status("active")
            .build();

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
            .with_endpoint(endpoint_ref.clone())
            .with_managing_organization(managing_org_ref.clone())
            .with_part_of(part_of_ref.clone())
            .build();

        let actual = location.get_references();

        assert_eq!(expected, actual)
    }
}

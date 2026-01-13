use crate::r4::{
    elements::{
        Address, BackboneElement, CodeableConcept, Coding, ContactPoint, DaysOfWeek, Identifier,
        Reference,
    },
    resources::{
        DomainResource, Endpoint, HoursOfOperation, Location, LocationPosition, LocationStatus,
        Organization, ResourceType,
    },
};

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

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.backbone_element.element.id = Some(id.into());
        self
    }

    pub fn longitude(mut self, longitude: f32) -> Self {
        self.longitude = longitude;
        self
    }

    pub fn latitude(mut self, latitude: f32) -> Self {
        self.latitude = latitude;
        self
    }

    pub fn altitude(mut self, altitude: f32) -> Self {
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

#[derive(Default)]
pub struct HoursOfOperationBuilder {
    backbone_element: BackboneElement,
    days_of_week: Option<Vec<DaysOfWeek>>,
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

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.backbone_element.element.id = Some(id.into());
        self
    }
    pub fn days_of_week(mut self, days: Vec<DaysOfWeek>) -> Self {
        self.days_of_week = Some(days);
        self
    }

    pub fn add_day_of_week(mut self, day: DaysOfWeek) -> Self {
        match &mut self.days_of_week {
            Some(dow) => dow.push(day.into()),
            None => self.days_of_week = Some(vec![day.into()]),
        }
        self
    }

    pub fn all_day(mut self, value: bool) -> Self {
        self.all_day = Some(value);
        self
    }

    pub fn operation_time(mut self, time: impl Into<String>) -> Self {
        self.operation_time = Some(time.into());
        self
    }

    pub fn closing_time(mut self, time: impl Into<String>) -> Self {
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

pub struct LocationBuilder {
    domain_resource: DomainResource,
    identifier: Option<Vec<Identifier>>,
    status: Option<LocationStatus>,
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
    resource_type: String,
}

impl Default for LocationBuilder {
    fn default() -> Self {
        LocationBuilder {
            resource_type: Location::get_resource_type(),
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

impl LocationBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.domain_resource.resource.id = Some(id.into());
        builder
    }

    pub fn identifier(mut self, identifier: Vec<Identifier>) -> Self {
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
    pub fn status(mut self, status: LocationStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn operational_status(mut self, coding: Coding) -> Self {
        self.operational_status = Some(coding);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn alias(mut self, aliases: Vec<String>) -> Self {
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

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn mode(mut self, mode: impl Into<String>) -> Self {
        self.mode = Some(mode.into());
        self
    }

    pub fn r#type(mut self, r#type: Vec<CodeableConcept>) -> Self {
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

    pub fn telecom(mut self, telecom: Vec<ContactPoint>) -> Self {
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

    pub fn address(mut self, address: Address) -> Self {
        self.address = Some(address);
        self
    }

    pub fn physical_type(mut self, concept: CodeableConcept) -> Self {
        self.physical_type = Some(concept);
        self
    }

    pub fn position(mut self, position: LocationPosition) -> Self {
        self.position = Some(position);
        self
    }

    pub fn managing_organization(mut self, org: Reference<Organization>) -> Self {
        self.managing_organization = Some(org);
        self
    }

    pub fn part_of(mut self, part_of: Reference<Location>) -> Self {
        self.part_of = Some(part_of);
        self
    }

    pub fn hours_of_operation(mut self, hours: Vec<HoursOfOperation>) -> Self {
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

    pub fn availability_exceptions(mut self, text: impl Into<String>) -> Self {
        self.availability_exceptions = Some(text.into());
        self
    }

    pub fn endpoint(mut self, endpoint: Reference<Endpoint>) -> Self {
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
            resource_type: self.resource_type,
        }
    }
}

#[cfg(test)]
mod test {

    use crate::r4::resources::Resource;

    use super::*;

    #[test]
    fn test_build_location_position_should_succeed() {
        let mut expected = LocationPosition::default();
        expected.backbone_element.element.id = Some("position-1".to_string());
        expected.latitude = 10.00;
        expected.longitude = 20.00;
        expected.altitude = Some(20.00);

        let actual = LocationPositionBuilder::default()
            .id("position-1")
            .latitude(10.00)
            .longitude(20.00)
            .altitude(20.00)
            .build();

        assert_eq!(actual, expected);
    }
    #[test]
    fn test_build_hours_of_operation_should_succeed() {
        let mut expected = HoursOfOperation::default();
        expected.days_of_week = Some(vec![DaysOfWeek::Mon, DaysOfWeek::Thu]);
        expected.all_day = Some(true);
        expected.operation_time = Some("all day".to_string());
        expected.closing_time = Some("midnight".to_string());

        let actual = HoursOfOperationBuilder::default()
            .add_day_of_week(DaysOfWeek::Mon)
            .add_day_of_week(DaysOfWeek::Thu)
            .all_day(true)
            .operation_time("all day")
            .closing_time("midnight")
            .build();

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_build_location_shoud_succeed() {
        let expected = Location {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("location-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            alias: Some(vec!["alias".to_string()]),
            address: Some(Address {
                state: Some("Arizona".to_string()),
                r#use: Some("official".to_string()),
                ..Default::default()
            }),

            description: Some("text".to_string()),
            status: Some(LocationStatus::Active),
            ..Default::default()
        };
        let actual_address = Address {
            r#use: Some("official".to_string()),

            state: Some("Arizona".to_string()),
            ..Default::default()
        };
        let actual = LocationBuilder::new("location-1")
            .alias(vec!["alias".to_string()])
            .address(actual_address)
            .description("text")
            .status(LocationStatus::Active)
            .build();

        assert_eq!(expected, actual)
    }
}

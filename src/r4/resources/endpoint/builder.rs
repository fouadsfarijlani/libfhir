use crate::r4::{
    elements::{CodeableConcept, Coding, ContactPoint, Identifier, Period, Reference},
    resources::{DomainResource, Endpoint, EndpointStatus, Organization, Resource, ResourceType},
};

pub struct EndpointBuilder {
    domain_resource: DomainResource,
    identifier: Option<Vec<Identifier>>,
    status: EndpointStatus,
    connection_type: Coding,
    managing_organization: Option<Reference<Organization>>,
    contact: Option<Vec<ContactPoint>>,
    period: Option<Period>,
    payload_type: Vec<CodeableConcept>,
    payload_mime_type: Option<Vec<String>>, // to be resolved later
    address: String,                        // to be resolved later
    header: Option<Vec<String>>,
    resource_type: String,
}

impl Default for EndpointBuilder {
    fn default() -> Self {
        EndpointBuilder {
            domain_resource: DomainResource {
                ..Default::default()
            },
            identifier: None,
            status: EndpointStatus::Test,
            connection_type: Coding {
                ..Default::default()
            },
            managing_organization: None,
            contact: None,
            period: None,
            payload_type: vec![],
            payload_mime_type: None,
            address: "".to_string(),
            header: None,
            resource_type: Endpoint::get_resource_type(),
        }
    }
}

impl EndpointBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        EndpointBuilder {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some(id.into()),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.domain_resource.resource.id = Some(id.into());
        self
    }

    pub fn identifier(mut self, indentifier: Vec<Identifier>) -> Self {
        self.identifier = Some(indentifier);
        self
    }

    pub fn add_identifier(mut self, identifier: Identifier) -> Self {
        match &mut self.identifier {
            Some(ident) => ident.push(identifier),
            None => self.identifier = Some(vec![identifier]),
        }
        self
    }

    pub fn status(mut self, status: EndpointStatus) -> Self {
        self.status = status;
        self
    }

    pub fn connection_type(mut self, connection_type: Coding) -> Self {
        self.connection_type = connection_type;
        self
    }

    pub fn managing_organization(mut self, managing_organization: Reference<Organization>) -> Self {
        self.managing_organization = Some(managing_organization);
        self
    }

    pub fn contact(mut self, contact: Vec<ContactPoint>) -> Self {
        self.contact = Some(contact);
        self
    }

    pub fn add_conctact(mut self, contact: ContactPoint) -> Self {
        match &mut self.contact {
            Some(c) => c.push(contact),
            None => self.contact = Some(vec![contact]),
        }
        self
    }

    pub fn payload_type(mut self, payload_type: Vec<CodeableConcept>) -> Self {
        self.payload_type = payload_type;
        self
    }

    pub fn add_payload_type(mut self, payload_type: CodeableConcept) -> Self {
        self.payload_type.push(payload_type);
        self
    }

    pub fn period(mut self, period: Period) -> Self {
        self.period = Some(period);
        self
    }

    pub fn payload_mime_type(mut self, payload_mime_type: Vec<String>) -> Self {
        self.payload_mime_type = Some(payload_mime_type);
        self
    }

    pub fn add_mime_type(mut self, playload_mime_type: impl Into<String>) -> Self {
        match &mut self.payload_mime_type {
            Some(pmt) => pmt.push(playload_mime_type.into()),
            None => self.payload_mime_type = Some(vec![playload_mime_type.into()]),
        }
        self
    }

    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = address.into();
        self
    }

    pub fn header(mut self, header: Vec<String>) -> Self {
        self.header = Some(header);
        self
    }

    pub fn add_header(mut self, header: impl Into<String>) -> Self {
        match &mut self.header {
            Some(h) => h.push(header.into()),
            None => self.header = Some(vec![header.into()]),
        }
        self
    }

    pub fn build(self) -> Endpoint {
        Endpoint {
            domain_resource: self.domain_resource,
            identifier: self.identifier,
            status: self.status,
            connection_type: self.connection_type,
            managing_organization: self.managing_organization,
            contact: self.contact,
            period: self.period,
            payload_type: self.payload_type,
            payload_mime_type: self.payload_mime_type,
            address: self.address,
            header: self.header,
            resource_type: self.resource_type,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::r4::{
        elements::{CodeableConceptBuilder, CodingBuilder, ReferenceBuilder},
        resources::Resource,
    };

    use super::*;

    #[test]
    fn test_build_should_succeed() {
        let connection_type = CodingBuilder::default().with_code("a code").build();
        let managing_org = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();
        let payload_type = CodeableConceptBuilder::default().with_text("mime").build();

        let expected = Endpoint {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("endpoint-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            status: EndpointStatus::Test,
            connection_type: connection_type.clone(),
            address: "http://example.com".to_string(),
            managing_organization: Some(managing_org.clone()),
            header: Some(vec!["content-type: application/json".to_string()]),
            payload_type: vec![payload_type.clone()],
            ..Default::default()
        };

        let actual = EndpointBuilder::new("endpoint-1")
            .status(EndpointStatus::Test)
            .connection_type(connection_type)
            .managing_organization(managing_org)
            .add_header("content-type: application/json")
            .add_payload_type(payload_type)
            .address("http://example.com")
            .build();

        assert_eq!(expected, actual)
    }
}

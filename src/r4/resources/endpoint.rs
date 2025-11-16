use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        CodeableConcept, Coding, ContactPoint, GetResourceReferences, Identifier, Period,
        Reference, ReferenceTypes,
    },
    resources::{DomainResource, Organization, ResourceType},
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case", serialize = "lowercase"))]
pub enum EndpointStatus {
    Active,
    Suspended,
    Error,
    Off,
    EnteredInError,
    Test,
}

impl Default for EndpointStatus {
    fn default() -> Self {
        EndpointStatus::Test
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Endpoint {
    #[serde(flatten)]
    pub domain_resource: DomainResource,
    pub identifier: Option<Vec<Identifier>>,
    pub status: EndpointStatus,
    pub connection_type: Coding,
    pub managing_organization: Option<Reference<Organization>>,
    pub contact: Option<Vec<ContactPoint>>,
    pub period: Option<Period>,
    pub payload_type: Vec<CodeableConcept>,
    pub payload_mime_type: Option<Vec<String>>, // to be resolved later
    pub address: String,                        // to be resolved later
    pub header: Option<String>,
}

impl Endpoint {
    pub fn from_json(data: &str) -> Self {
        let result = serde_json::from_str(data);
        match result {
            Ok(ep) => ep,
            Err(e) => panic!("{e:?}"),
        }
    }
}

impl ResourceType for Endpoint {
    const TYPE: &'static str = "Endpoint";
}

impl GetResourceReferences for Endpoint {
    fn get_references(&self) -> Vec<ReferenceTypes> {
        let mut references: Vec<ReferenceTypes> = Vec::new();
        if let Some(managing_organization) = &self.managing_organization {
            references.push(ReferenceTypes::from(managing_organization));
        }

        references
    }
}

#[derive(Default)]
pub struct EndpointBuilder {
    domain_resource: DomainResource,
    identifier: Option<Vec<Identifier>>,
    status: EndpointStatus, // to be resolved later
    connection_type: Coding,
    managing_organization: Option<Reference<Organization>>,
    contact: Option<Vec<ContactPoint>>,
    period: Option<Period>,
    payload_type: Vec<CodeableConcept>,
    payload_mime_type: Option<Vec<String>>, // to be resolved later
    address: String,                        // to be resolved later
    header: Option<String>,
}

impl EndpointBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.domain_resource.resource.id = Some(id.into());
        builder
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.domain_resource.resource.id = Some(id.into());
        self
    }

    pub fn with_identifier(mut self, indentifier: Vec<Identifier>) -> Self {
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

    pub fn with_status(mut self, status: EndpointStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_connection_type(mut self, connection_type: Coding) -> Self {
        self.connection_type = connection_type;
        self
    }

    pub fn with_managing_organization(
        mut self,
        managing_organization: Reference<Organization>,
    ) -> Self {
        self.managing_organization = Some(managing_organization);
        self
    }

    pub fn with_contact(mut self, contact: Vec<ContactPoint>) -> Self {
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

    pub fn with_payload_type(mut self, payload_type: Vec<CodeableConcept>) -> Self {
        self.payload_type = payload_type;
        self
    }

    pub fn add_payload_type(mut self, payload_type: CodeableConcept) -> Self {
        self.payload_type.push(payload_type);
        self
    }

    pub fn with_period(mut self, period: Period) -> Self {
        self.period = Some(period);
        self
    }

    pub fn with_payload_mime_type(mut self, payload_mime_type: Vec<String>) -> Self {
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

    pub fn with_address(mut self, address: impl Into<String>) -> Self {
        self.address = address.into();
        self
    }

    pub fn with_header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
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
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        elements::{
            CodeableConceptBuilder, CodingBuilder, ContactPointBuilder, IdentifierBuilder,
            PeriodBuilder, ReferenceBuilder,
        },
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
                    meta: None,
                    implicit_rules: None,
                },
                text: None,
                contained: None,
                exnetions: None,
            },
            status: EndpointStatus::Test,
            connection_type: connection_type.clone(),
            address: "http://example.com".to_string(),
            managing_organization: Some(managing_org.clone()),
            identifier: None,
            header: Some("content-type: application/json".to_string()),
            period: None,
            payload_type: vec![payload_type.clone()],
            payload_mime_type: None,
            contact: None,
        };

        let actual = EndpointBuilder::new("endpoint-1")
            .with_status(EndpointStatus::Test)
            .with_connection_type(connection_type)
            .with_managing_organization(managing_org)
            .with_header("content-type: application/json")
            .add_payload_type(payload_type)
            .with_address("http://example.com")
            .build();

        assert_eq!(expected, actual)
    }
    #[test]
    fn from_json_should_succeed() {
        let data = r#"
        {
            "resourceType":"Endpoint",
            "id": "endpoint-1",
            "status": "error",
            "identifier": [
                {
                    "use": "official",
                    "system": "http://example.com",
                    "value": "ep-system-1"
                }
            ],
            "connectionType": {
                "system": "some-system",
                "code": "some-code"
            },
            "managingOrganization": {
                "reference": "Organization/1"
            },
            "contact": [
                {
                    "system": "http://example.com",
                    "value": "contact-value-1"
                }
            ],
            "period": {
                "start": "2025-01-01",
                "end": "2026-01-01"
            },
            "payloadType": [
                {
                    "coding":[
                        {
                            "system": "some-system"
                        }
                    ],
                    "text": "some text"
                }
            ],
            "address": "http://example.com",
            "header": "Accept: application/json"
        }
        "#;
        let connection_type = CodingBuilder::default()
            .with_system("some-system")
            .with_code("some-code")
            .build();
        let managing_org = ReferenceBuilder::default()
            .with_reference("Organization/1")
            .build::<Organization>();
        let identifer = IdentifierBuilder::default()
            .with_system("http://example.com")
            .with_use("official")
            .with_value("ep-system-1")
            .build();
        let coding = CodingBuilder::default().with_system("some-system").build();
        let payload_entry = CodeableConceptBuilder::default()
            .with_text("some text")
            .add_coding(coding)
            .build();
        let payload_type = vec![payload_entry];
        let contact_point = ContactPointBuilder::default()
            .with_system("http://example.com")
            .with_value("contact-value-1")
            .build();
        let period = PeriodBuilder::default()
            .with_start("2025-01-01")
            .with_end("2026-01-01")
            .build();

        let expected = EndpointBuilder::new("endpoint-1")
            .with_status(EndpointStatus::Error)
            .with_connection_type(connection_type)
            .with_payload_type(payload_type)
            .with_managing_organization(managing_org)
            .add_conctact(contact_point)
            .with_address("http://example.com")
            .add_identifier(identifer)
            .with_period(period)
            .with_header("Accept: application/json")
            .build();

        let actual = Endpoint::from_json(data);

        assert_eq!(expected, actual);
    }
}

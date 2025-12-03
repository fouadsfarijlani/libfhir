use serde::{Deserialize, Serialize};

use crate::r4::{
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    pub status: EndpointStatus,

    pub connection_type: Coding,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub managing_organization: Option<Reference<Organization>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Vec<ContactPoint>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
    pub payload_type: Vec<CodeableConcept>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_mime_type: Option<Vec<String>>, // to be resolved later

    pub address: String, // to be resolved later

    #[serde(skip_serializing_if = "Option::is_none")]
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::r4::{
        elements::{
            CodeableConceptBuilder, CodingBuilder, ContactPointBuilder, IdentifierBuilder,
            PeriodBuilder, ReferenceBuilder,
        },
        resources::EndpointBuilder,
    };

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

use crate::r4::{
    elements::{
        Address, Attachment, BackboneElement, CodeableConcept, ContactPoint, HumanName,
        Identifier, Period, Reference,
    },
    resources::{
        AdministrativeGender, DomainResource, Organization, Patient, PatientCommunication,
        PatientContact, PatientLink, Practitioner, Resource, ResourceType,
    },
};

#[derive(Default)]
pub struct PatientContactBuilder {
    backbone_element: BackboneElement,
    relationship: Option<Vec<CodeableConcept>>,
    name: Option<HumanName>,
    telecom: Option<Vec<ContactPoint>>,
    address: Option<Address>,
    gender: Option<String>,
    organization: Option<Reference<Organization>>,
    period: Option<Period>,
}

impl PatientContactBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.backbone_element.element.id = Some(id.into());
        builder
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.backbone_element.element.id = Some(id.into());
        self
    }

    pub fn relationship(mut self, relationship: Vec<CodeableConcept>) -> Self {
        self.relationship = Some(relationship);
        self
    }

    pub fn add_relationship(mut self, relationship: CodeableConcept) -> Self {
        match &mut self.relationship {
            Some(rel) => rel.push(relationship),
            None => self.relationship = Some(vec![relationship]),
        }
        self
    }

    pub fn name(mut self, name: HumanName) -> Self {
        self.name = Some(name);
        self
    }

    pub fn telecom(mut self, telecom: Vec<ContactPoint>) -> Self {
        self.telecom = Some(telecom);
        self
    }

    pub fn add_telecom(mut self, telecom: ContactPoint) -> Self {
        match &mut self.telecom {
            Some(tm) => tm.push(telecom),
            None => self.telecom = Some(vec![telecom]),
        }
        self
    }

    pub fn address(mut self, address: Address) -> Self {
        self.address = Some(address);
        self
    }

    pub fn gender(mut self, gender: impl Into<String>) -> Self {
        self.gender = Some(gender.into());
        self
    }

    pub fn organization(mut self, organization: Reference<Organization>) -> Self {
        self.organization = Some(organization);
        self
    }

    pub fn period(mut self, period: Period) -> Self {
        self.period = Some(period);
        self
    }

    pub fn build(self) -> PatientContact {
        PatientContact {
            backbone_element: self.backbone_element,
            relationship: self.relationship,
            name: self.name,
            telecom: self.telecom,
            address: self.address,
            gender: self.gender,
            organization: self.organization,
            period: self.period,
        }
    }
}

#[derive(Default)]
pub struct PatientCommunicationBuilder {
    backbone_element: BackboneElement,
    language: CodeableConcept,
    preferred: Option<bool>,
}

impl PatientCommunicationBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.backbone_element.element.id = Some(id.into());
        builder
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.backbone_element.element.id = Some(id.into());
        self
    }

    pub fn language(mut self, language: CodeableConcept) -> Self {
        self.language = language;
        self
    }

    pub fn preferred(mut self, preferred: bool) -> Self {
        self.preferred = Some(preferred);
        self
    }

    pub fn build(self) -> PatientCommunication {
        PatientCommunication {
            backbone_element: self.backbone_element,
            language: self.language,
            preferred: self.preferred,
        }
    }
}

#[derive(Default)]
pub struct PatientLinkBuilder {
    backbone_element: BackboneElement,
    other: Reference<Patient>,
    r#type: String,
}

impl PatientLinkBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.backbone_element.element.id = Some(id.into());
        builder
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.backbone_element.element.id = Some(id.into());
        self
    }

    pub fn other(mut self, other: Reference<Patient>) -> Self {
        self.other = other;
        self
    }

    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = r#type.into();
        self
    }

    pub fn build(self) -> PatientLink {
        PatientLink {
            backbone_element: self.backbone_element,
            other: self.other,
            r#type: self.r#type,
        }
    }
}

#[derive(Default)]
pub struct PatientBuilder {
    domain_resource: DomainResource,
    identifier: Option<Vec<Identifier>>,
    active: Option<bool>,
    name: Option<Vec<HumanName>>,
    telecom: Option<Vec<ContactPoint>>,
    gender: Option<AdministrativeGender>,
    birth_date: Option<String>,
    deceased_boolean: Option<bool>,
    deceased_date_time: Option<String>,
    address: Option<Vec<Address>>,
    marital_status: Option<CodeableConcept>,
    multiple_birth_boolean: Option<bool>,
    multiple_birth_integer: Option<u32>,
    photo: Option<Vec<Attachment>>,
    contact: Option<Vec<PatientContact>>,
    communication: Option<Vec<PatientCommunication>>,
    general_practitioner: Option<Vec<Reference<Practitioner>>>,
    managing_organization: Option<Reference<Organization>>,
    link: Option<Vec<PatientLink>>,
}

impl PatientBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        PatientBuilder {
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

    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    pub fn name(mut self, name: Vec<HumanName>) -> Self {
        self.name = Some(name);
        self
    }

    pub fn add_name(mut self, name: HumanName) -> Self {
        match &mut self.name {
            Some(n) => n.push(name),
            None => self.name = Some(vec![name]),
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

    pub fn gender(mut self, gender: AdministrativeGender) -> Self {
        self.gender = Some(gender);
        self
    }

    pub fn birth_date(mut self, birth_date: impl Into<String>) -> Self {
        self.birth_date = Some(birth_date.into());
        self
    }

    pub fn deceased_boolean(mut self, deceased_boolean: bool) -> Self {
        self.deceased_boolean = Some(deceased_boolean);
        self
    }

    pub fn deceased_date_time(mut self, deceased_date_time: impl Into<String>) -> Self {
        self.deceased_date_time = Some(deceased_date_time.into());
        self
    }

    pub fn address(mut self, address: Vec<Address>) -> Self {
        self.address = Some(address);
        self
    }

    pub fn add_address(mut self, address: Address) -> Self {
        match &mut self.address {
            Some(a) => a.push(address),
            None => self.address = Some(vec![address]),
        }
        self
    }

    pub fn marital_status(mut self, marital_status: CodeableConcept) -> Self {
        self.marital_status = Some(marital_status);
        self
    }

    pub fn multiple_birth_boolean(mut self, multiple_birth_boolean: bool) -> Self {
        self.multiple_birth_boolean = Some(multiple_birth_boolean);
        self
    }

    pub fn multiple_birth_integer(mut self, multiple_birth_integer: u32) -> Self {
        self.multiple_birth_integer = Some(multiple_birth_integer);
        self
    }

    pub fn photo(mut self, photo: Vec<Attachment>) -> Self {
        self.photo = Some(photo);
        self
    }

    pub fn add_photo(mut self, photo: Attachment) -> Self {
        match &mut self.photo {
            Some(p) => p.push(photo),
            None => self.photo = Some(vec![photo]),
        }
        self
    }

    pub fn contact(mut self, contact: Vec<PatientContact>) -> Self {
        self.contact = Some(contact);
        self
    }

    pub fn add_contact(mut self, contact: PatientContact) -> Self {
        match &mut self.contact {
            Some(c) => c.push(contact),
            None => self.contact = Some(vec![contact]),
        }
        self
    }

    pub fn communication(mut self, communication: Vec<PatientCommunication>) -> Self {
        self.communication = Some(communication);
        self
    }

    pub fn add_communication(mut self, communication: PatientCommunication) -> Self {
        match &mut self.communication {
            Some(c) => c.push(communication),
            None => self.communication = Some(vec![communication]),
        }
        self
    }

    pub fn general_practitioner(mut self, general_practitioner: Vec<Reference<Practitioner>>) -> Self {
        self.general_practitioner = Some(general_practitioner);
        self
    }

    pub fn add_general_practitioner(mut self, general_practitioner: Reference<Practitioner>) -> Self {
        match &mut self.general_practitioner {
            Some(gp) => gp.push(general_practitioner),
            None => self.general_practitioner = Some(vec![general_practitioner]),
        }
        self
    }

    pub fn managing_organization(mut self, managing_organization: Reference<Organization>) -> Self {
        self.managing_organization = Some(managing_organization);
        self
    }

    pub fn link(mut self, link: Vec<PatientLink>) -> Self {
        self.link = Some(link);
        self
    }

    pub fn add_link(mut self, link: PatientLink) -> Self {
        match &mut self.link {
            Some(l) => l.push(link),
            None => self.link = Some(vec![link]),
        }
        self
    }

    pub fn build(self) -> Patient {
        Patient {
            domain_resource: self.domain_resource,
            identifier: self.identifier,
            active: self.active,
            name: self.name,
            telecom: self.telecom,
            gender: self.gender,
            birth_date: self.birth_date,
            deceased_boolean: self.deceased_boolean,
            deceased_date_time: self.deceased_date_time,
            address: self.address,
            marital_status: self.marital_status,
            multiple_birth_boolean: self.multiple_birth_boolean,
            multiple_birth_integer: self.multiple_birth_integer,
            photo: self.photo,
            contact: self.contact,
            communication: self.communication,
            general_practitioner: self.general_practitioner,
            managing_organization: self.managing_organization,
            link: self.link,
            resource_type: Patient::get_resource_type(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::r4::{
        elements::{Coding, CodeableConceptBuilder, ContactPointBuilder, Element, HumanNameBuilder, ReferenceBuilder},
        resources::{OrganizationBuilder, PatientBuilder, PatientContactBuilder, PatientCommunicationBuilder, PatientLinkBuilder, PractitionerBuilder},
    };

    use super::*;

    #[test]
    pub fn test_build_patient_should_succeed() {
        let expected = Patient {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("patient-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            identifier: None,
            active: Some(true),
            name: Some(vec![HumanName {
                family: Some("Doe".to_string()),
                given: Some(vec!["John".to_string()]),
                ..Default::default()
            }]),
            telecom: None,
            gender: Some(AdministrativeGender::Male),
            birth_date: Some("1980-01-01".to_string()),
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
            resource_type: Patient::get_resource_type(),
        };

        let human_name = HumanNameBuilder::default()
            .with_family("Doe")
            .with_given(vec!["John".to_string()])
            .build();

        let actual = PatientBuilder::new("patient-1")
            .active(true)
            .gender(AdministrativeGender::Male)
            .birth_date("1980-01-01")
            .add_name(human_name)
            .build();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_build_patient_contact_should_succeed() {
        let expected = PatientContact {
            backbone_element: BackboneElement {
                element: Element {
                    id: Some("contact-1".to_string()),
                    extention: None,
                },
                modifier_extensions: None,
            },
            relationship: Some(vec![CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string()),
                    code: Some("EMERGENCY".to_string()),
                    display: Some("Emergency Contact".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }]),
            name: Some(HumanName {
                family: Some("Smith".to_string()),
                given: Some(vec!["Jane".to_string()]),
                ..Default::default()
            }),
            telecom: Some(vec![ContactPoint {
                system: Some("phone".to_string()),
                value: Some("+1-555-123-4567".to_string()),
                r#use: Some("mobile".to_string()),
                ..Default::default()
            }]),
            address: None,
            gender: None,
            organization: None,
            period: None,
        };

        let relationship = CodeableConceptBuilder::default()
            .add_coding(
                Coding {
                    system: Some(
                        "http://terminology.hl7.org/CodeSystem/v3-RoleCode".to_string(),
                    ),
                    code: Some("EMERGENCY".to_string()),
                    display: Some("Emergency Contact".to_string()),
                    ..Default::default()
                },
            )
            .build();

        let human_name = HumanNameBuilder::default()
            .with_family("Smith")
            .with_given(vec!["Jane".to_string()])
            .build();

        let telecom = ContactPointBuilder::default()
            .with_system("phone")
            .with_value("+1-555-123-4567")
            .with_use("mobile")
            .build();

        let actual = PatientContactBuilder::new("contact-1")
            .relationship(vec![relationship])
            .name(human_name)
            .telecom(vec![telecom])
            .build();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_build_patient_communication_should_succeed() {
        let expected = PatientCommunication {
            backbone_element: BackboneElement {
                element: Element {
                    id: Some("comm-1".to_string()),
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
                ..Default::default()
            },
            preferred: Some(true),
        };

        let language = CodeableConceptBuilder::default()
            .add_coding(
                Coding {
                    system: Some("urn:ietf:bcp:47".to_string()),
                    code: Some("en".to_string()),
                    display: Some("English".to_string()),
                    ..Default::default()
                },
            )
            .build();

        let actual = PatientCommunicationBuilder::new("comm-1")
            .language(language)
            .preferred(true)
            .build();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_build_patient_link_should_succeed() {
        let other_patient_ref = ReferenceBuilder::default()
            .with_reference("Patient/other-patient")
            .build::<Patient>();

        let expected = PatientLink {
            backbone_element: BackboneElement {
                element: Element {
                    id: Some("link-1".to_string()),
                    extention: None,
                },
                modifier_extensions: None,
            },
            other: other_patient_ref.clone(),
            r#type: "seealso".to_string(),
        };

        let actual = PatientLinkBuilder::new("link-1")
            .other(other_patient_ref)
            .r#type("seealso")
            .build();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_build_patient_with_all_fields_should_succeed() {
        let org_ref = ReferenceBuilder::default()
            .with_reference("Organization/org-1")
            .build::<Organization>();

        let prac_ref = ReferenceBuilder::default()
            .with_reference("Practitioner/prac-1")
            .build::<Practitioner>();

        let human_name = HumanNameBuilder::default()
            .with_family("Doe")
            .with_given(vec!["John".to_string()])
            .build();

        let telecom = ContactPointBuilder::default()
            .with_system("phone")
            .with_value("+1-555-123-4567")
            .with_use("home")
            .build();

        let address = Address {
            r#use: Some("home".to_string()),
            line: Some(vec!["123 Main St".to_string()]),
            city: Some("Springfield".to_string()),
            ..Default::default()
        };

        let identifier = Identifier {
            r#use: Some("official".to_string()),
            system: Some("http://example.org/patients".to_string()),
            value: Some("PAT-001".to_string()),
            ..Default::default()
        };

        let marital_status = CodeableConceptBuilder::default()
            .with_text("Married")
            .build();

        let attachment = Attachment {
            content_type: Some("image/jpeg".to_string()),
            url: Some("https://example.org/photo.jpg".to_string()),
            ..Default::default()
        };

        let contact = PatientContactBuilder::new("contact-1")
            .name(human_name.clone())
            .add_telecom(telecom.clone())
            .build();

        let communication = PatientCommunicationBuilder::new("comm-1")
            .language(
                CodeableConceptBuilder::default()
                    .with_text("English")
                    .build(),
            )
            .preferred(true)
            .build();

        let link = PatientLinkBuilder::new("link-1")
            .other(
                ReferenceBuilder::default()
                    .with_reference("Patient/other")
                    .build::<Patient>(),
            )
            .r#type("seealso")
            .build();

        let expected = Patient {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("patient-full".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            identifier: Some(vec![identifier.clone()]),
            active: Some(true),
            name: Some(vec![human_name.clone()]),
            telecom: Some(vec![telecom.clone()]),
            gender: Some(AdministrativeGender::Male),
            birth_date: Some("1980-01-01".to_string()),
            deceased_boolean: Some(false),
            deceased_date_time: None,
            address: Some(vec![address.clone()]),
            marital_status: Some(marital_status.clone()),
            multiple_birth_boolean: Some(false),
            multiple_birth_integer: None,
            photo: Some(vec![attachment.clone()]),
            contact: Some(vec![contact.clone()]),
            communication: Some(vec![communication.clone()]),
            general_practitioner: Some(vec![prac_ref.clone()]),
            managing_organization: Some(org_ref.clone()),
            link: Some(vec![link.clone()]),
            resource_type: Patient::get_resource_type(),
        };

        let actual = PatientBuilder::new("patient-full")
            .active(true)
            .gender(AdministrativeGender::Male)
            .birth_date("1980-01-01")
            .deceased_boolean(false)
            .add_identifier(identifier)
            .add_name(human_name)
            .add_telecom(telecom)
            .add_address(address)
            .marital_status(marital_status)
            .multiple_birth_boolean(false)
            .add_photo(attachment)
            .add_contact(contact)
            .add_communication(communication)
            .add_general_practitioner(prac_ref)
            .managing_organization(org_ref)
            .add_link(link)
            .build();

        assert_eq!(expected, actual)
    }
}

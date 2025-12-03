use crate::r4::{
    elements::{
        Address, Attachement, BackboneElement, CodeableConcept, ContactPoint, Element, HumanName,
        Identifier, Period, Reference,
    },
    resources::{
        DomainResource, Gender, Organization, Practitioner, PractitionerQualification, Resource,
        ResourceType,
    },
};

#[derive(Default)]
pub struct PractitionerQualificationBuilder {
    backbone_element: BackboneElement,
    identifier: Option<Vec<Identifier>>,
    code: CodeableConcept,
    period: Option<Period>,
    issuer: Option<Reference<Organization>>,
}

impl PractitionerQualificationBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        PractitionerQualificationBuilder {
            backbone_element: BackboneElement {
                element: Element {
                    id: Some(id.into()),
                    extention: None,
                },
                modifier_extensions: None,
            },
            ..Default::default()
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.backbone_element.element.id = Some(id.into());
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

    pub fn code(mut self, code: CodeableConcept) -> Self {
        self.code = code;
        self
    }

    pub fn period(mut self, period: Period) -> Self {
        self.period = Some(period);
        self
    }

    pub fn issuer(mut self, issuer: Reference<Organization>) -> Self {
        self.issuer = Some(issuer);
        self
    }

    pub fn build(self) -> PractitionerQualification {
        PractitionerQualification {
            backbone_element: self.backbone_element,
            identifier: self.identifier,
            code: self.code,
            period: self.period,
            issuer: self.issuer,
        }
    }
}

#[derive(Default)]
pub struct PractitionerBuilder {
    domain_resource: DomainResource,
    identifier: Option<Vec<Identifier>>,
    active: Option<bool>,
    name: Option<Vec<HumanName>>,
    telecom: Option<Vec<ContactPoint>>,
    address: Option<Vec<Address>>,
    gender: Option<Gender>,     // TODO: This should be a code
    birth_date: Option<String>, // Date for later
    photo: Option<Vec<Attachement>>,
    qualification: Option<Vec<PractitionerQualification>>,
    communication: Option<Vec<CodeableConcept>>,
}

impl PractitionerBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        PractitionerBuilder {
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

    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn name(mut self, names: Vec<HumanName>) -> Self {
        self.name = Some(names);
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

    pub fn address(mut self, addresses: Vec<Address>) -> Self {
        self.address = Some(addresses);
        self
    }

    pub fn add_address(mut self, address: Address) -> Self {
        match &mut self.address {
            Some(a) => a.push(address),
            None => self.address = Some(vec![address]),
        }
        self
    }

    pub fn gender(mut self, gender: Gender) -> Self {
        self.gender = Some(gender);
        self
    }

    pub fn birth_date(mut self, date: impl Into<String>) -> Self {
        self.birth_date = Some(date.into());
        self
    }

    pub fn photo(mut self, photos: Vec<Attachement>) -> Self {
        self.photo = Some(photos);
        self
    }

    pub fn add_photo(mut self, photo: Attachement) -> Self {
        match &mut self.photo {
            Some(p) => p.push(photo),
            None => self.photo = Some(vec![photo]),
        }
        self
    }

    pub fn qualification(mut self, qualifications: Vec<PractitionerQualification>) -> Self {
        self.qualification = Some(qualifications);
        self
    }

    pub fn add_qualification(mut self, qualification: PractitionerQualification) -> Self {
        match &mut self.qualification {
            Some(q) => q.push(qualification),
            None => self.qualification = Some(vec![qualification]),
        }
        self
    }

    pub fn communication(mut self, communication: Vec<CodeableConcept>) -> Self {
        self.communication = Some(communication);
        self
    }

    pub fn add_communication(mut self, communication: CodeableConcept) -> Self {
        match &mut self.communication {
            Some(q) => q.push(communication),
            None => self.communication = Some(vec![communication]),
        }
        self
    }

    pub fn build(self) -> Practitioner {
        Practitioner {
            domain_resource: self.domain_resource,
            identifier: self.identifier,
            active: self.active,
            name: self.name,
            telecom: self.telecom,
            address: self.address,
            gender: self.gender,
            birth_date: self.birth_date,
            photo: self.photo,
            qualification: self.qualification,
            communication: self.communication,
            resource_type: Practitioner::get_resource_type(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::r4::elements::CodeableConceptBuilder;

    use super::*;

    #[test]
    fn test_build_should_succeed() {
        let expected = Practitioner {
            qualification: Some(vec![PractitionerQualification {
                code: CodeableConcept {
                    text: Some("very qualified".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            }]),
            ..Default::default()
        };
        let qualification = PractitionerQualificationBuilder::default()
            .code(
                CodeableConceptBuilder::default()
                    .with_text("very qualified")
                    .build(),
            )
            .build();

        let actual = PractitionerBuilder::default()
            .add_qualification(qualification)
            .build();

        assert_eq!(expected, actual)
    }
}

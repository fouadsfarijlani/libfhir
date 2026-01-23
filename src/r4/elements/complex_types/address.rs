use serde::{Deserialize, Serialize};

use crate::{
    FhirError,
    r4::{
        elements::{CodeableConcept, Element, Period},
        resources::ResourceType,
    },
};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Address {
    #[serde(flatten)]
    pub element: Element,
    pub r#use: Option<String>, // to be resolved
    pub r#type: Option<CodeableConcept>,
    pub text: Option<String>,
    pub line: Option<Vec<String>>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub period: Option<Period>,
}

impl ResourceType for Address {
    const TYPE: &'static str = "Address";
}

impl Address {
    pub fn from_json(data: &str) -> Result<Self, FhirError> {
        Ok(serde_json::from_str(data)?)
    }
}

#[derive(Default)]
pub struct AddressBuilder {
    element: Element,
    r#use: Option<String>, // to be resolved
    r#type: Option<CodeableConcept>,
    text: Option<String>,
    line: Option<Vec<String>>,
    city: Option<String>,
    district: Option<String>,
    state: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
    period: Option<Period>,
}

impl AddressBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut address_builder = Self::default();
        address_builder.element.id = Some(id.into());
        address_builder
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.element.id = Some(id.into());
        self
    }

    pub fn with_use(mut self, r#use: impl Into<String>) -> Self {
        self.r#use = Some(r#use.into());
        self
    }

    pub fn with_type(mut self, r#type: CodeableConcept) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_line(mut self, line: Vec<String>) -> Self {
        self.line = Some(line);
        self
    }

    pub fn add_line(mut self, line: impl Into<String>) -> Self {
        match &mut self.line {
            None => self.line = Some(vec![line.into()]),
            Some(lines) => lines.push(line.into()),
        }
        self
    }

    pub fn with_city(mut self, city: impl Into<String>) -> Self {
        self.city = Some(city.into());
        self
    }

    pub fn with_district(mut self, district: impl Into<String>) -> Self {
        self.district = Some(district.into());
        self
    }

    pub fn with_state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    pub fn with_postal_code(mut self, postal_code: impl Into<String>) -> Self {
        self.postal_code = Some(postal_code.into());
        self
    }

    pub fn with_country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    pub fn with_period(mut self, period: Period) -> Self {
        self.period = Some(period);
        self
    }

    pub fn build(self) -> Address {
        Address {
            element: self.element,
            r#use: self.r#use,
            r#type: self.r#type,
            text: self.text,
            line: self.line,
            city: self.city,
            district: self.district,
            state: self.state,
            postal_code: self.postal_code,
            country: self.country,
            period: self.period,
        }
    }
}
#[cfg(test)]
mod test {
    use crate::r4::elements::{CodeableConceptBuilder, CodingBuilder, PeriodBuilder};

    use super::*;

    #[test]
    fn test_build_should_succeed() {
        let expected = Address {
            element: Element {
                id: Some("some-address-id".to_string()),
                extention: None,
            },
            r#use: Some("official".to_string()),
            r#type: Some(
                CodeableConceptBuilder::new("some-id")
                    .add_coding(
                        CodingBuilder::new("some-id")
                            .with_version("some-version")
                            .build(),
                    )
                    .build(),
            ),
            text: Some("text".to_string()),
            line: Some(vec!["some-line".to_string()]),
            city: Some("gondor".to_string()),
            district: Some("some-district".to_string()),
            state: Some("the-state".to_string()),
            postal_code: Some("12345".to_string()),
            country: Some("middle-earth".to_string()),
            period: None,
        };

        let coding = CodingBuilder::new("some-id")
            .with_version("some-version")
            .build();
        let address_type = CodeableConceptBuilder::new("some-id")
            .add_coding(coding)
            .build();
        let actual = AddressBuilder::new("some-address-id")
            .with_use("official")
            .with_type(address_type)
            .with_text("text")
            .with_line(vec!["some-line".to_string()])
            .with_city("gondor")
            .with_district("some-district")
            .with_state("the-state")
            .with_postal_code("12345")
            .with_country("middle-earth")
            .build();

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_from_json_should_suceed() {
        let data = r#"
            {
                "resourceType": "Address",
                "id": "address-1",
                "use": "official",
                "type": {
                    "id": "type-1",
                    "coding": [
                        {
                            "id": "coding-id",
                            "system": "http://example.org",
                            "version": "the version",
                            "code": "important code",
                            "userSelected": false
                            
                        }
                    ],
                    "text": "nice text"
                },
                "text": "some other fancy text",
                "line": ["i walk the line"],
                "city": "New York",
                "district": "nine nine",
                "state": "de staat",
                "postalCode": "123 ABC",
                "country": "Never Never Land",
                "period": {
                    "start": "20-12-2024",
                    "end": "20-12-2025"
                } 
            }
        "#;

        let period = PeriodBuilder::default()
            .with_start("20-12-2024")
            .with_end("20-12-2025")
            .build();

        let expected = AddressBuilder::new("address-1")
            .with_use("official")
            .with_type(
                CodeableConceptBuilder::new("type-1")
                    .add_coding(
                        CodingBuilder::new("coding-id")
                            .with_system("http://example.org")
                            .with_version("the version")
                            .with_code("important code")
                            .with_user_selected(false)
                            .build(),
                    )
                    .with_text("nice text")
                    .build(),
            )
            .with_text("some other fancy text")
            .add_line("i walk the line")
            .with_city("New York")
            .with_district("nine nine")
            .with_state("de staat")
            .with_postal_code("123 ABC")
            .with_country("Never Never Land")
            .with_period(period)
            .build();

        let actual = Address::from_json(data).unwrap();
        assert_eq!(expected, actual)
    }
}

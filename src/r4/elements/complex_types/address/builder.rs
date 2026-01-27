use crate::r4::elements::{Address, CodeableConcept, Element, Period};

#[derive(Default)]
pub struct AddressBuilder {
    element: Element,
    r#use: Option<String>,
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
        let mut builder = Self::default();
        builder.element.id = Some(id.into());
        builder
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.element.id = Some(id.into());
        self
    }

    pub fn r#use(mut self, r#use: impl Into<String>) -> Self {
        self.r#use = Some(r#use.into());
        self
    }

    pub fn r#type(mut self, r#type: CodeableConcept) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn line(mut self, line: Vec<String>) -> Self {
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

    pub fn city(mut self, city: impl Into<String>) -> Self {
        self.city = Some(city.into());
        self
    }

    pub fn district(mut self, district: impl Into<String>) -> Self {
        self.district = Some(district.into());
        self
    }

    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    pub fn postal_code(mut self, postal_code: impl Into<String>) -> Self {
        self.postal_code = Some(postal_code.into());
        self
    }

    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    pub fn period(mut self, period: Period) -> Self {
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
    use crate::r4::elements::{CodeableConceptBuilder, CodingBuilder};

    use super::*;

    #[test]
    fn test_builder_should_succeed() {
        let coding = CodingBuilder::new("coding-id").with_version("v1").build();

        let address_type = CodeableConceptBuilder::new("type-id")
            .add_coding(coding)
            .build();

        let expected = Address {
            element: Element {
                id: Some("addr-1".to_string()),
                ..Default::default()
            },
            r#use: Some("official".to_string()),
            r#type: Some(address_type.clone()),
            text: Some("Some text".to_string()),
            line: Some(vec!["Line 1".to_string()]),
            city: Some("Gondor".to_string()),
            state: Some("Middle Earth".to_string()),
            country: Some("Arda".to_string()),
            ..Default::default()
        };

        let actual = AddressBuilder::new("addr-1")
            .r#use("official")
            .r#type(address_type)
            .text("Some text")
            .add_line("Line 1")
            .city("Gondor")
            .state("Middle Earth")
            .country("Arda")
            .build();

        assert_eq!(expected, actual);
    }
}

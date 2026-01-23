use serde::{Deserialize, Serialize};

use crate::{
    FhirError,
    r4::{
        elements::{Element, Period},
        resources::ResourceType,
    },
};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct HumanName {
    #[serde(flatten)]
    pub element: Element,
    pub r#use: Option<String>, // to be resolved
    pub text: Option<String>,
    pub family: Option<String>,
    pub given: Option<Vec<String>>,
    pub prefix: Option<Vec<String>>,
    pub suffix: Option<Vec<String>>,
    pub period: Option<Period>,
}

impl ResourceType for HumanName {
    const TYPE: &'static str = "HumanName";
}

impl HumanName {
    pub fn from_json(data: &str) -> Result<Self, FhirError> {
        Ok(serde_json::from_str(data)?)
    }
}

#[derive(Default)]
pub struct HumanNameBuilder {
    element: Element,
    r#use: Option<String>, // to be resolved
    text: Option<String>,
    family: Option<String>,
    given: Option<Vec<String>>,
    prefix: Option<Vec<String>>,
    suffix: Option<Vec<String>>,
    period: Option<Period>,
}

impl HumanNameBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut human_name_builder = HumanNameBuilder::default();
        human_name_builder.element.id = Some(id.into());
        human_name_builder
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.element.id = Some(id.into());
        self
    }
    pub fn with_use(mut self, r#use: impl Into<String>) -> Self {
        self.r#use = Some(r#use.into());
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_family(mut self, family: impl Into<String>) -> Self {
        self.family = Some(family.into());
        self
    }

    pub fn with_given(mut self, given: Vec<String>) -> Self {
        self.given = Some(given);
        self
    }

    pub fn add_given(mut self, given: impl Into<String>) -> Self {
        match &mut self.given {
            Some(g) => g.push(given.into()),
            None => self.given = Some(vec![given.into()]),
        }
        self
    }

    pub fn with_prefix(mut self, prefix: Vec<String>) -> Self {
        self.prefix = Some(prefix);
        self
    }

    pub fn add_prefix(mut self, prefix: impl Into<String>) -> Self {
        match &mut self.prefix {
            Some(p) => p.push(prefix.into()),
            None => self.prefix = Some(vec![prefix.into()]),
        }
        self
    }

    pub fn with_suffix(mut self, suffix: Vec<String>) -> Self {
        self.suffix = Some(suffix);
        self
    }

    pub fn add_suffix(mut self, suffix: impl Into<String>) -> Self {
        match &mut self.suffix {
            Some(s) => s.push(suffix.into()),
            None => self.suffix = Some(vec![suffix.into()]),
        }
        self
    }

    pub fn with_period(mut self, period: Period) -> Self {
        self.period = Some(period);
        self
    }

    pub fn build(self) -> HumanName {
        HumanName {
            element: self.element,
            r#use: self.r#use,
            text: self.text,
            family: self.family,
            given: self.given,
            prefix: self.prefix,
            suffix: self.suffix,
            period: self.period,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::r4::elements::PeriodBuilder;

    use super::*;

    #[test]
    fn test_from_json_should_succeed() {
        let data = r#"
        {
            "id": "123",
            "use": "official",
            "text": "Dr. John Smith",
            "family": "Smith",
            "given": ["John"],
            "prefix": ["Dr."],
            "suffix": ["PhD"],
            "period": { "start": "2020-01-01", "end": "2030-01-01" }
        }
        "#;
        let period = PeriodBuilder::default()
            .with_start("2020-01-01")
            .with_end("2030-01-01")
            .build();
        let expected = HumanNameBuilder::default()
            .with_id("123".to_string())
            .with_use("official")
            .with_text("Dr. John Smith")
            .with_family("Smith")
            .add_given("John")
            .add_prefix("Dr.")
            .add_suffix("PhD")
            .with_period(period)
            .build();

        let actual = HumanName::from_json(data).unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_build_should_succeed() {
        let expected = HumanName {
            element: Element {
                id: Some("123".to_string()),
                extention: None,
            },
            r#use: Some("official".to_string()),
            text: Some("John Doe".to_string()),
            family: Some("Doe".to_string()),
            given: Some(vec!["John".to_string()]),
            prefix: Some(vec!["Dr.".to_string()]),
            suffix: Some(vec!["PhD".to_string()]),
            period: Some(Period {
                element: Element {
                    id: None,
                    extention: None,
                },
                start: Some("2020-01-01".to_string()),
                end: None,
            }),
        };

        let period = PeriodBuilder::default().with_start("2020-01-01").build();
        let actual = HumanNameBuilder::new("123")
            .with_use("official")
            .with_text("John Doe")
            .with_family("Doe")
            .add_given("John")
            .add_prefix("Dr.")
            .add_suffix("PhD")
            .with_period(period)
            .build();

        assert_eq!(expected, actual)
    }
}

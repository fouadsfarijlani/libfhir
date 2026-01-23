use serde::{Deserialize, Serialize};

use crate::{
    FhirError,
    r4::{
        elements::{Element, Period},
        resources::ResourceType,
    },
};

// TODO: Add ContactPointSystem
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct ContactPoint {
    #[serde(flatten)]
    pub element: Element,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>, // to be resolved,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#use: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
}

impl ResourceType for ContactPoint {
    const TYPE: &'static str = "ContactPoint";
}

impl ContactPoint {
    pub fn from_json(data: &str) -> Result<Self, FhirError> {
        Ok(serde_json::from_str(data)?)
    }
}

#[derive(Default)]
pub struct ContactPointBuilder {
    element: Element,
    system: Option<String>, // to be resolved,
    value: Option<String>,
    r#use: Option<String>,
    rank: Option<u32>,
    period: Option<Period>,
}

impl ContactPointBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut contact_point_builder = ContactPointBuilder::default();
        contact_point_builder.element.id = Some(id.into());
        contact_point_builder
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.element.id = Some(id.into());
        self
    }
    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_use(mut self, r#use: impl Into<String>) -> Self {
        self.r#use = Some(r#use.into());
        self
    }

    pub fn with_rank(mut self, rank: u32) -> Self {
        self.rank = Some(rank);
        self
    }

    pub fn with_period(mut self, period: Period) -> Self {
        self.period = Some(period);
        self
    }

    pub fn build(self) -> ContactPoint {
        ContactPoint {
            element: self.element,
            system: self.system,
            value: self.value,
            r#use: self.r#use,
            rank: self.rank,
            period: self.period,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::r4::elements::PeriodBuilder;

    use super::*;

    #[test]
    fn from_json_should_succeed() {
        let data = r#"
            {
               "resourceType": "ContactPoint",
               "id": "contact-point-1",
               "system": "a fancy system",
               "value": "the value",
               "use": "official",
               "rank": 1,
               "period": {
                    "start": "01-01-2025",
                    "end": "01-01-2026"
               }
            }
        "#;
        let period = PeriodBuilder::default()
            .with_start("01-01-2025")
            .with_end("01-01-2026")
            .build();
        let expected = ContactPointBuilder::new("contact-point-1")
            .with_system("a fancy system")
            .with_value("the value")
            .with_use("official")
            .with_rank(1)
            .with_period(period)
            .build();

        let actual = ContactPoint::from_json(data).unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_build_should_succeed() {
        let expected = ContactPoint {
            element: Element {
                id: Some("contact-point-1".to_string()),
                extention: None,
            },
            system: Some("a fancy system".to_string()),
            value: Some("a value".to_string()),
            r#use: Some("big use".to_string()),
            rank: Some(1),
            period: Some(Period {
                element: Element {
                    id: None,
                    extention: None,
                },
                start: Some("10-10-2025".to_string()),
                end: Some("10-10-2026".to_string()),
            }),
        };

        let period = PeriodBuilder::default()
            .with_start("10-10-2025")
            .with_end("10-10-2026")
            .build();
        let actual = ContactPointBuilder::new("contact-point-1")
            .with_system("a fancy system")
            .with_value("a value")
            .with_use("big use")
            .with_rank(1)
            .with_period(period)
            .build();

        assert_eq!(expected, actual)
    }
}

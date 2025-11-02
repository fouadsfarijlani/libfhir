use serde::{Deserialize, Serialize};

use crate::{
    elements::Element,
    resources::{self, ResourceType},
};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Period {
    #[serde(flatten)]
    pub element: Element,
    pub start: Option<String>, // to br resolved later,
    pub end: Option<String>,   // to be resolved later
}

impl ResourceType for Period {
    const TYPE: &'static str = "Period";
}

impl Period {
    pub fn from_json(data: &str) -> Self {
        resources::from_json(data)
    }
}

#[derive(Default)]
pub struct PeriodBuilder {
    element: Element,
    start: Option<String>, // to br resolved later,
    end: Option<String>,   // to be resolved later
}

impl PeriodBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut period_builder = Self::default();
        period_builder.element.id = Some(id.into());
        period_builder
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.element.id = Some(id.into());
        self
    }

    pub fn with_start(mut self, start: impl Into<String>) -> Self {
        self.start = Some(start.into());
        self
    }

    pub fn with_end(mut self, end: impl Into<String>) -> Self {
        self.end = Some(end.into());
        self
    }

    pub fn build(self) -> Period {
        Period {
            element: self.element,
            start: self.start,
            end: self.end,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_json_should_succeed() {
        let data = r#"
            {
                "resourceType": "Period",
                "id": "period-1",
                "start": "10-10-2010",
                "end": "20-10-2025"
            }
        "#;
        let expected = PeriodBuilder::new("period-1")
            .with_start("10-10-2010")
            .with_end("20-10-2025")
            .build();

        let actual = Period::from_json(data);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_build_should_succeed() {
        let expected = Period {
            element: Element {
                id: Some("period-1".to_string()),
                extention: None,
            },
            start: Some("10-10-2010".to_string()),
            end: Some("10-10-2020".to_string()),
        };

        let actual = PeriodBuilder::new("period-1")
            .with_start("10-10-2010")
            .with_end("10-10-2020")
            .build();

        assert_eq!(expected, actual)
    }
}

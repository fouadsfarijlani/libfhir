use serde::{Deserialize, Serialize};

use crate::{
    FhirError,
    r4::{
        elements::{BackboneElement, Period},
        resources::ResourceType,
    },
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct NotAvailable {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,
    pub description: String,
    pub during: Option<Period>,
}

impl ResourceType for NotAvailable {
    const TYPE: &'static str = "NotAvailable";
}

impl NotAvailable {
    pub fn from_json(data: &str) -> Result<Self, FhirError> {
        Ok(serde_json::from_str(data)?)
    }
}

#[derive(Default)]
pub struct NotAvailableBuilder {
    backbone_element: BackboneElement,
    description: String,
    during: Option<Period>,
}

impl NotAvailableBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.backbone_element.element.id = Some(id.into());
        builder
    }

    pub fn with_desscription(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_during(mut self, during: Period) -> Self {
        self.during = Some(during);
        self
    }

    pub fn build(self) -> NotAvailable {
        NotAvailable {
            backbone_element: self.backbone_element,
            description: self.description,
            during: self.during,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::r4::elements::{Element, PeriodBuilder};

    use super::*;

    #[test]
    fn test_from_json_should_succeed() {
        let data = r#"
        {
            "id": "nv-1",
            "description": "just because we dont want to work",
            "during": {
               "start": "08:00",
               "end": "17:00"
            }
        }
        "#;

        let expected = NotAvailable {
            backbone_element: BackboneElement {
                element: Element {
                    id: Some("nv-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            description: "just because we dont want to work".to_string(),
            during: Some(Period {
                element: Element {
                    ..Default::default()
                },
                start: Some("08:00".to_string()),
                end: Some("17:00".to_string()),
            }),
        };

        let actual = NotAvailable::from_json(data).unwrap();
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_build_should_succeed() {
        let expected = NotAvailable {
            backbone_element: BackboneElement {
                element: Element {
                    id: Some("nv-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            description: "lots of text".to_string(),
            during: Some(Period {
                element: Element {
                    ..Default::default()
                },
                start: Some("08:00".to_string()),
                end: Some("17:00".to_string()),
            }),
        };

        let actual = NotAvailableBuilder::new("nv-1")
            .with_desscription("lots of text")
            .with_during(
                PeriodBuilder::default()
                    .with_start("08:00")
                    .with_end("17:00")
                    .build(),
            )
            .build();

        assert_eq!(expected, actual)
    }
}

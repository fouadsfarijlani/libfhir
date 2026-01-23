use serde::{Deserialize, Serialize};

use crate::{
    FhirError,
    r4::{
        elements::{Coding, Element},
        resources::ResourceType,
    },
};

#[derive(Debug, Default, Serialize, PartialEq, Deserialize, Clone)]
pub struct CodeableConcept {
    #[serde(flatten)]
    pub element: Element,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding: Option<Vec<Coding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl ResourceType for CodeableConcept {
    const TYPE: &'static str = "CodeableConcept";
}

impl CodeableConcept {
    pub fn from_json(data: &str) -> Result<Self, FhirError> {
        Ok(serde_json::from_str(data)?)
    }
}

#[derive(Default)]
pub struct CodeableConceptBuilder {
    element: Element,
    coding: Option<Vec<Coding>>,
    text: Option<String>,
}

impl CodeableConceptBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut codeable_concept = Self::default();
        codeable_concept.element.id = Some(id.into());
        codeable_concept
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.element.id = Some(id.into());
        self
    }

    pub fn with_coding(mut self, coding: Vec<Coding>) -> Self {
        self.coding = Some(coding);
        self
    }

    pub fn add_coding(mut self, coding: Coding) -> Self {
        match &mut self.coding {
            None => self.coding = Some(vec![coding]),
            Some(codings) => codings.push(coding),
        }
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn build(self) -> CodeableConcept {
        CodeableConcept {
            element: self.element,
            coding: self.coding,
            text: self.text,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::r4::elements::CodingBuilder;

    use super::*;

    #[test]
    fn test_build_should_succeed() {
        let expected_coding = CodingBuilder::new("some-id").with_code("some-code").build();
        let expected = CodeableConcept {
            element: Element {
                id: Some("some-id".to_string()),
                extention: None,
            },
            coding: Some(vec![expected_coding]),
            text: Some("some-text".to_string()),
        };

        let actual = CodeableConceptBuilder::new("some-id")
            .with_coding(vec![
                CodingBuilder::new("some-id").with_code("some-code").build(),
            ])
            .with_text("some-text")
            .build();

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_from_json_should_succeed() {
        let data = r#"
            {
                "id": "some-id",
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
            }
        "#;
        let expected = CodeableConceptBuilder::new("some-id")
            .with_text("nice text")
            .add_coding(
                CodingBuilder::new("coding-id")
                    .with_system("http://example.org")
                    .with_version("the version")
                    .with_code("important code")
                    .with_user_selected(false)
                    .build(),
            )
            .build();

        let actual = CodeableConcept::from_json(data).unwrap();

        assert_eq!(expected, actual)
    }
}

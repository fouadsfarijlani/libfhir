use serde::{Deserialize, Serialize};

use crate::{
    elements::Element,
    resources::{self, ResourceType},
};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Coding {
    #[serde(flatten)]
    pub element: Element,
    pub system: Option<String>, // to be resolved later
    pub version: Option<String>,
    pub code: Option<String>, // to be resolved later
    pub user_selected: Option<bool>,
}

impl ResourceType for Coding {
    const TYPE: &'static str = "Coding";
}

impl Coding {
    pub fn from_json(data: &str) -> Self {
        resources::resource::from_json(data)
    }
}

#[derive(Default)]
pub struct CodingBuilder {
    element: Element,
    system: Option<String>,
    version: Option<String>,
    code: Option<String>,
    user_selected: Option<bool>,
}

impl CodingBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut coding = CodingBuilder::default();
        coding.element.id = Some(id.into());
        coding
    }

    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn with_user_selected(mut self, user_selected: bool) -> Self {
        self.user_selected = Some(user_selected);
        self
    }

    pub fn build(self) -> Coding {
        Coding {
            element: self.element,
            system: self.system,
            version: self.version,
            code: self.code,
            user_selected: self.user_selected,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build_should_succeed() {
        let expected = Coding {
            element: Element {
                id: Some("some-id".to_string()),
                extention: None,
            },
            system: Some("some-system".to_string()),
            version: Some("some-version".to_string()),
            code: Some("some-code".to_string()),
            user_selected: Some(true),
        };

        let actual = CodingBuilder::new("some-id")
            .with_system("some-system")
            .with_version("some-version")
            .with_code("some-code")
            .with_user_selected(true)
            .build();

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_from_json_should_succeed() {
        let data = r#"
        {
            "id": "some-id",
            "system": "http://example.org",
            "version": "the version",
            "code": "important code",
            "userSelected": false
        }
        "#;
        let expected = CodingBuilder::new("some-id")
            .with_system("http://example.org")
            .with_version("the version")
            .with_code("important code")
            .with_user_selected(false)
            .build();

        let actual = Coding::from_json(data);

        assert_eq!(expected, actual)
    }
}

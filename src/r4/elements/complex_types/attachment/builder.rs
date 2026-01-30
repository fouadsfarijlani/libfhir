use crate::r4::elements::{Attachment, Element};

#[derive(Default)]
pub struct AttachmentBuilder {
    element: Element,
    content_type: Option<String>,
    language: Option<String>,
    data: Option<String>,
    url: Option<String>,
    size: Option<u32>,
    hash: Option<String>,
    title: Option<String>,
    creation: Option<String>,
}

impl AttachmentBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.element.id = Some(id.into());
        builder
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.element.id = Some(id.into());
        self
    }

    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn hash(mut self, hash: impl Into<String>) -> Self {
        self.hash = Some(hash.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn creation(mut self, creation: impl Into<String>) -> Self {
        self.creation = Some(creation.into());
        self
    }

    pub fn build(self) -> Attachment {
        Attachment {
            element: self.element,
            content_type: self.content_type,
            language: self.language,
            data: self.data,
            url: self.url,
            size: self.size,
            hash: self.hash,
            title: self.title,
            creation: self.creation,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build_should_succeed() {
        let expected = Attachment {
            element: Element {
                id: Some("attachment-1".to_string()),
                extention: None,
            },
            content_type: Some("image/png".to_string()),
            language: Some("nl".to_string()),
            data: Some("data".to_string()),
            url: Some("https://example.org".to_string()),
            size: Some(5432),
            hash: Some("hash".to_string()),
            title: Some("Patient Photo".to_string()),
            creation: Some("2025-11-07T14:23:00Z".to_string()),
        };

        let actual = AttachmentBuilder::new("attachment-1")
            .content_type("image/png")
            .language("nl")
            .data("data")
            .url("https://example.org")
            .size(5432)
            .hash("hash")
            .title("Patient Photo")
            .creation("2025-11-07T14:23:00Z")
            .build();

        assert_eq!(expected, actual);
    }
}

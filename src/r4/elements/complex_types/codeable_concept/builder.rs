use crate::r4::elements::{CodeableConcept, Coding, Element};

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

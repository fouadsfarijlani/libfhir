use crate::Element;

#[derive(Debug)]
pub struct CodeableConcept {
    elements: Option<Element>,
    coding: Option<Vec<String>>, // to be resolved later
    test: Option<String>,
}

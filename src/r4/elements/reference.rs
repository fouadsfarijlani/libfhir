use crate::elements::{complex_types::Identifier, element::Element};

#[derive(Debug)]
pub struct Reference {
    element: Option<Element>,
    reference: Option<String>,
    r#type: Option<String>,
    display: Option<String>,
    identifier: Option<Vec<Identifier>>,
}

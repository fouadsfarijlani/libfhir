use crate::CodeableConcept;
use crate::Element;
use crate::Reference;

#[derive(Debug)]
pub struct Identifier {
    element: Option<Element>,
    r#use: Option<String>,
    r#type: Option<CodeableConcept>,
    system: Option<String>,
    value: Option<String>,
    period: Option<String>, // to be resolved
    assigner: Option<Reference>,
}

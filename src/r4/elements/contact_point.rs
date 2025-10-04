use crate::Element;

#[derive(Debug)]
pub struct ContactPoint {
    element: Element,
    system: Option<String>,
    value: Option<String>,
    rank: Option<u32>,
    period: Option<String>, // to be resolved later
}

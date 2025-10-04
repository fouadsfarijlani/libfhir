#[derive(Debug)]
pub struct Element {
    id: Option<String>,

    // Complex: Extention [0..*]
    extention: Option<Vec<String>>, // to be resolved later
}

#[derive(Debug)]
pub struct Resource {
    id: Option<String>,
    meta: Option<String>, // to be resoloved later
    implicit_rules: Option<String>,
}

#[derive(Debug)]
pub struct DomainResource {
    resource: Option<Resource>,
    text: Option<String>,
    contained: Option<Vec<String>>, // To be resolved later
    exnetions: Option<Vec<String>>, // to be resolved later
}

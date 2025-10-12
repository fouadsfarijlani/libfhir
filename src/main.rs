use libfhir::r4::resources::Organization;

mod fhir;

fn main() {
    let example = r#"
    {
    "resourceType": "Organization",
    "id": "organization-1",
    "name": "example organization",
    "part_of": {"reference": "Organization/big-corp-1"},
    "endpoint": [{"reference": "Endpoint/ep-default-1"}]
    }
    "#;
    let org = Organization::from_json(example);
    println!("{:?}", org.get_references())
}

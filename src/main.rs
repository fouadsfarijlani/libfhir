use libfhir::r4::resources::OrganizationBuilder;

mod fhir;

fn main() {
    let org = OrganizationBuilder::new("org-1")
        .with_active(true)
        .add_alias("some alias")
        .with_name("some-name")
        .build();

    println!("{:?}", org.to_json_string())
}

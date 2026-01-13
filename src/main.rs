use libfhir::r4::resources::OrganizationBuilder;

mod fhir;

fn main() {
    let org = OrganizationBuilder::new("org-1")
        .active(true)
        .add_alias("some alias")
        .name("some-name")
        .build();

    println!("{:?}", org.to_json_string())
}

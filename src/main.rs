use libfhir::resources::LocationPosition;

mod fhir;

fn main() {
    let position = LocationPosition::default();
    println!("{:?}", position)
}

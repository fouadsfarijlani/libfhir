pub mod r4 {
    pub mod elements {
        pub mod complex_types;
        pub mod element;
        pub mod reference;
    }
    pub mod resources {
        pub mod endpoint;
        pub mod organization;
        pub use organization::*;

        pub mod organization_affiliation;
        pub mod resource;
    }
}

pub use r4::*;
